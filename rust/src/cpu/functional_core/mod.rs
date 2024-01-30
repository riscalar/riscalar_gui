mod execute;
mod stats;
use self::stats::ProcessorStats;
use super::{
    csr::ControlStatusRegisters,
    exception::Exception,
    memory::{Memory, MEM_BASE},
    register::XRegisterFile,
    stats::Tick,
};

pub struct FunctionalCore {
    pub xreg: XRegisterFile,
    pub csrs: ControlStatusRegisters,
    pub pc: u64,
    pub mode: Mode,
    pub memory: Memory,
    pub stats: ProcessorStats,
    pub tick: Tick,
}

impl FunctionalCore {
    pub fn new() -> Self {
        Self {
            xreg: XRegisterFile::new(),
            csrs: ControlStatusRegisters::new(),
            pc: MEM_BASE,
            mode: Mode::Machine,
            // bus: Bus::new(),
            memory: Memory::new(),
            stats: ProcessorStats::new(),
            tick: 0,
        }
    }

    pub fn reset(&mut self) {
        self.stats = ProcessorStats::new();
        self.csrs = ControlStatusRegisters::new();
        self.xreg = XRegisterFile::new();
        self.pc = 0;
    }

    pub fn read(&mut self, addr: u64, size: u8) -> Result<u64, Exception> {
        self.stats.mem.mem_data_reads += 1;
        self.memory.read(addr, size)
    }

    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        self.stats.mem.mem_data_writes += 1;
        self.memory.write(addr, value, size)
    }

    pub fn fetch(&mut self) -> Result<u64, Exception> {
        self.stats.mem.mem_inst_reads += 1;
        match self.memory.read(self.pc, 32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Exception::InstructionAccessFault),
        }
    }

    pub fn run(
        &mut self,
        num_instrs_to_execute: Option<u64>,
    ) -> Result<RunSuccessStatus, Exception> {
        let mut num_instrs_executed = 0;
        loop {
            let inst = match self.fetch() {
                Ok(inst) => inst,
                Err(e) => match e {
                    Exception::InstructionAccessFault => break,
                    _ => return Err(e),
                },
            };
            match self.execute(inst) {
                Ok(()) => (),
                Err(e) => match e {
                    Exception::InstructionAccessFault => break,
                    _ => return Err(e),
                },
            };
            self.tick += 1;
            num_instrs_executed += 1;
            if let Some(num_instrs) = num_instrs_to_execute {
                if num_instrs_executed >= num_instrs {
                    return Ok(RunSuccessStatus::EarlyTermination);
                }
            }
        }
        Ok(RunSuccessStatus::Complete)
    }
}

#[derive(Clone)]
pub enum Mode {
    Machine,
}

#[derive(Debug)]
pub enum RunSuccessStatus {
    Complete,
    EarlyTermination,
}
