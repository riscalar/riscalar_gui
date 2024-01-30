// #![allow(dead_code, unused_variables)]
pub mod components;
mod execute;
mod instr;
pub mod latency_args;
mod reservation_station;
mod stage1_fetch;
mod stage2_dispatch;
mod stage3_issue;
mod stage4_writeback;
mod stage5_commit;
mod stats;

use self::{
    components::{
        branch_predictor::BranchPredictor,
        cache::{CacheHierachy, MemLatency},
        functional_unit_pool::FunctionalUnitPool,
    },
    latency_args::LatencyArgs,
    reservation_station::{DependencyLinker, RegisterCreator, ReservationStation},
    stage1_fetch::FetchItem,
    stage3_issue::ExecutionReadyQueue,
    stage4_writeback::EventQueue,
    stats::ProcessorStats,
};
use crate::cpu::{
    csr::ControlStatusRegisters,
    exception::Exception,
    functional_core::{FunctionalCore, Mode, RunSuccessStatus},
    memory::{Memory, MEM_BASE},
    register::{RegisterFile, XRegisterFile},
    stats::Tick,
};
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[derive(Clone, Copy)]
pub struct LatencyParameters {
    pub rsq_size: u16,
    pub lsq_size: u16,

    pub fetch_speed: u8,
    pub fetch_queue_size: u16,
    pub fetch_branch_penalty: Tick,

    pub decode_width: u8,

    pub issue_width: u8,
    pub issue_inorder: bool,
    pub issue_no_misspec: bool,

    pub commit_width: u8,

    pub cache_il1_lat: u8,
}

pub struct LatencyCore {
    pub xreg: XRegisterFile,
    pub csrs: ControlStatusRegisters,
    pub pc: u64,
    next_pc: u64,

    pub mode: Mode,
    pub memory: Memory,

    pub stats: ProcessorStats,
    pub tick: Tick,
    pub params: LatencyParameters,

    pred_pc: u64,
    recover_pc: u64,

    fetch_pc: u64,      /* Fetch Unit PC */
    fetch_pred_pc: u64, /* Fetch Unit Predicted PC, modified by branch predictor */

    fetch_dispatch_queue: VecDeque<FetchItem>,
    fetch_stall: bool,
    fetch_stall_cycles: Tick, /* Private variable to keep track of when/how many cycles the fetch has to stall*/

    rsq: VecDeque<Rc<ReservationStation>>,
    lsq: VecDeque<Rc<ReservationStation>>,

    pred_perfect: bool,
    branch_predictor: Option<Box<dyn BranchPredictor>>,

    inst_seq: u64,

    spec_mode: bool,
    spec_xregs: XRegisterFile,
    spec_xreg_valid: [bool; 32],
    spec_mem: HashMap<u64, u8>,

    register_creators: [RegisterCreator; 33],
    spec_register_creators: [RegisterCreator; 33],
    last_operation: DependencyLinker,

    ready_queue: ExecutionReadyQueue,
    event_queue: EventQueue,

    functional_unit_pool: FunctionalUnitPool,
    cache_hierachy: CacheHierachy,
    mem_latency: MemLatency,
}

impl LatencyCore {
    pub fn new(args: &LatencyArgs) -> Self {
        let cfg = LatencyParameters {
            rsq_size: args.rsq_size.unwrap_or(16),
            lsq_size: args.lsq_size.unwrap_or(8),

            fetch_speed: args.fetch_config.fetch_speed.unwrap_or(1),
            fetch_queue_size: args.fetch_config.fetch_queue_size.unwrap_or(4),
            fetch_branch_penalty: args.fetch_config.fetch_branch_penalty.unwrap_or(3),

            decode_width: args.decode_config.decode_width.unwrap_or(4),

            issue_width: args.issue_config.issue_width.unwrap_or(4),
            issue_inorder: match args.issue_config.issue_order {
                latency_args::IssueOrder::InOrder => true,
                latency_args::IssueOrder::OutOrder => false,
            },
            issue_no_misspec: args.issue_config.issue_no_misspec,

            commit_width: args.commit_config.commit_width.unwrap_or(4),
            cache_il1_lat: 1,
        };

        let fu_pool = FunctionalUnitPool::new_from_cfg(args.functional_unit_pool_config.clone());
        let cache_hierachy = CacheHierachy::new(args.cache_config.clone());

        Self {
            params: cfg,
            functional_unit_pool: fu_pool,
            cache_hierachy,
            mem_latency: MemLatency::new(args.memory_config.clone()),

            xreg: XRegisterFile::new(),
            csrs: ControlStatusRegisters::new(),

            mode: Mode::Machine,
            memory: Memory::new(),
            stats: ProcessorStats::new(),
            tick: 0,

            pc: MEM_BASE,
            next_pc: MEM_BASE + 4,
            pred_pc: MEM_BASE + 4,
            recover_pc: 0,

            fetch_pc: MEM_BASE,
            fetch_pred_pc: MEM_BASE,
            fetch_dispatch_queue: VecDeque::with_capacity(cfg.fetch_queue_size as usize + 1),
            fetch_stall: false,
            fetch_stall_cycles: 0,

            rsq: VecDeque::with_capacity(cfg.rsq_size as usize + 1),
            lsq: VecDeque::with_capacity(cfg.lsq_size as usize + 1),

            pred_perfect: true,
            branch_predictor: None,

            inst_seq: 0,

            spec_mode: false,
            spec_xregs: XRegisterFile::new(),
            spec_xreg_valid: [false; 32],
            spec_mem: HashMap::new(),

            register_creators: [(); 33].map(|_| RegisterCreator::null()),
            spec_register_creators: [(); 33].map(|_| RegisterCreator::null()),
            last_operation: DependencyLinker::null(),

            ready_queue: ExecutionReadyQueue::new((cfg.rsq_size + cfg.lsq_size) as usize),
            event_queue: EventQueue::new((cfg.rsq_size + cfg.lsq_size) as usize),
        }
    }

    pub fn from_functional_core(functional: &FunctionalCore, args: &LatencyArgs) -> Self {
        let mut latency_core = LatencyCore::new(args);
        latency_core.xreg = functional.xreg.clone();
        latency_core.csrs = functional.csrs.clone();

        latency_core.pc = functional.pc;
        latency_core.next_pc = functional.pc + 4;

        latency_core.mode = functional.mode.clone();
        latency_core.memory = functional.memory.clone();

        latency_core.fetch_pc = functional.pc - 4;
        latency_core.fetch_pred_pc = functional.pc;
        latency_core
    }

    pub fn reset(&mut self) {
        self.stats = ProcessorStats::new();
        self.csrs = ControlStatusRegisters::new();
        self.xreg = XRegisterFile::new();
        self.fetch_pc = 0;
    }

    pub fn read(&mut self, addr: u64, size: u8) -> Result<u64, Exception> {
        self.stats.mem.mem_data_reads += 1;
        if !Self::address_aligned(addr, size) {
            return Err(Exception::LoadAddressMisaligned);
        }
        match self.spec_mode {
            true => self.spec_read(addr, size),
            false => self.memory.read(addr, size),
        }
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        self.stats.mem.mem_data_writes += 1;
        if !Self::address_aligned(addr, size) {
            return Err(Exception::StoreAMOAddressMisaligned);
        }
        match self.spec_mode {
            true => self.spec_write(addr, value, size),
            false => self.memory.write(addr, value, size),
        }
    }

    pub fn fetch(&mut self) -> Result<u64, Exception> {
        self.stats.mem.mem_inst_reads += 1;
        match self.memory.read(self.fetch_pc, 32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Exception::InstructionAccessFault),
        }
    }

    fn address_aligned(addr: u64, size: u8) -> bool {
        match size {
            8 => true,
            16 => addr % 2 == 0,
            32 => addr % 4 == 0,
            64 => addr % 8 == 0,
            _ => true,
        }
    }

    fn xreg_read(&self, reg: u64) -> u64 {
        if self.spec_mode && self.spec_xreg_valid[reg as usize] {
            self.spec_xregs.read(reg)
        } else {
            self.xreg.read(reg)
        }
    }

    fn xreg_write(&mut self, reg: u64, value: u64) {
        if self.spec_mode {
            self.spec_xregs.write(reg, value);
            self.spec_xreg_valid[reg as usize] = true;
        } else {
            self.xreg.write(reg, value)
        }
    }

    pub fn run(&mut self, args: LatencyArgs) -> Result<RunSuccessStatus, Exception> {
        loop {
            self.run_commit_stage()?;
            self.functional_unit_pool.execute_cycle();
            self.run_writeback_stage()?;
            self.refresh_lsq()?;
            self.run_issue_stage()?;
            self.run_dispatch_stage()?;

            if self.fetch_stall_cycles == 0 {
                match self.run_fetch_stage() {
                    Ok(()) => (),
                    Err(e) => match e {
                        Exception::InstructionAccessFault => break,
                        _ => return Err(e),
                    },
                };
            } else {
                self.fetch_stall_cycles -= 1;
            }

            self.tick += 1;
            if let Some(max_instrs) = args.run_config.max_instrs {
                if self.stats.committed_instrs >= max_instrs as u128 {
                    return Ok(RunSuccessStatus::EarlyTermination);
                }
            }
        }
        self.stats.update_summary(self.tick);
        Ok(RunSuccessStatus::Complete)
    }
}
