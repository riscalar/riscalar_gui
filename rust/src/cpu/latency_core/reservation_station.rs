#![allow(clippy::too_many_arguments)]
use super::{
    instr::{instr_opcode, Opcode},
    LatencyCore,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservationStation {
    pub slip: u64,
    pub instr: u64,
    pub op: Cell<Opcode>,

    pub pc: u64,
    pub next_pc: u64,
    pub pred_pc: u64,

    pub in_lsq: bool,
    pub ea_comp: Cell<bool>,
    pub recover_instr: Cell<bool>,

    pub spec_mode: bool,
    pub addr: u64,
    pub tag: Cell<u32>,
    pub seq: u64,

    /* Instruction Status */
    pub queued: Cell<bool>,
    pub issued: Cell<bool>,
    pub completed: Cell<bool>,

    pub odep_name: Cell<u8>,
    pub odep_list: RefCell<Vec<DependencyLinker>>,

    pub idep_ready: Cell<(bool, bool)>,
}

impl ReservationStation {
    pub fn new_rsq_entry(
        slip: u64,
        instr: u64,
        pc: u64,
        next_pc: u64,
        pred_pc: u64,
        spec_mode: bool,
        tag: u32,
        seq: u64,
    ) -> Self {
        Self {
            slip,
            instr,
            op: Cell::new(instr_opcode(&instr).unwrap()),
            pc,
            next_pc,
            pred_pc,
            in_lsq: false,
            ea_comp: Cell::new(false),
            recover_instr: Cell::new(false),
            spec_mode,
            addr: 0,
            tag: Cell::new(tag),
            seq,
            queued: Cell::new(false),
            issued: Cell::new(false),
            completed: Cell::new(false),
            odep_name: Cell::new(0),
            odep_list: RefCell::new(Vec::new()),
            idep_ready: Cell::new((false, false)),
        }
    }

    pub fn new_lsq_entry(
        slip: u64,
        instr: u64,
        pc: u64,
        next_pc: u64,
        pred_pc: u64,
        spec_mode: bool,
        tag: u32,
        seq: u64,
        addr: u64,
    ) -> Self {
        Self {
            slip,
            instr,
            op: Cell::new(instr_opcode(&instr).unwrap()),
            pc,
            next_pc,
            pred_pc,
            in_lsq: true,
            ea_comp: Cell::new(false),
            recover_instr: Cell::new(false),
            spec_mode,
            addr,
            tag: Cell::new(tag),
            seq,
            queued: Cell::new(false),
            issued: Cell::new(false),
            completed: Cell::new(false),
            odep_name: Cell::new(0),
            odep_list: RefCell::new(Vec::new()),
            idep_ready: Cell::new((false, false)),
        }
    }

    pub fn addr_comput_op(&self) {
        self.ea_comp.set(true);
        self.op.set(Opcode::Imm);
    }

    pub fn set_recover_instr(&self) {
        self.recover_instr.set(true);
    }

    pub fn ideps_ready(&self) -> bool {
        self.idep_ready.get().0 && self.idep_ready.get().1
    }

    pub fn idep_ready(&self, idep_num: usize) -> bool {
        match idep_num {
            0 => self.idep_ready.get().0,
            1 => self.idep_ready.get().1,
            _ => panic!("Invalid idep_num"),
        }
    }

    pub fn set_completed(&self) {
        self.completed.set(true);
    }

    pub fn set_issued(&self) {
        self.issued.set(true);
    }

    pub fn set_queued(&self) {
        self.queued.set(true);
    }

    pub fn unqueue(&self) {
        self.queued.set(false);
    }

    pub fn invalidate(&self) {
        self.tag.set(self.tag.get() + 1);
    }

    pub fn set_idep_ready(&self, idep_num: usize) {
        match idep_num {
            0 => self.idep_ready.set((true, self.idep_ready.get().1)),
            1 => self.idep_ready.set((self.idep_ready.get().0, true)),
            _ => panic!("Invalid idep_num"),
        }
    }

    pub fn set_has_idep(&self, idep_num: usize) {
        match idep_num {
            0 => self.idep_ready.set((false, self.idep_ready.get().1)),
            1 => self.idep_ready.set((self.idep_ready.get().0, false)),
            _ => panic!("Invalid idep_num"),
        }
    }

    pub fn set_odep_name(&self, odep_name: u8) {
        self.odep_name.set(odep_name);
    }

    pub fn set_no_odep(&self) {
        self.odep_name.set(0);
    }

    pub fn add_odep(&self, consumer: DependencyLinker) {
        self.odep_list.borrow_mut().push(consumer);
    }

    pub fn store_op_ready(&self) -> bool {
        self.idep_ready.get().0
    }

    pub fn store_addr_ready(&self) -> bool {
        self.idep_ready.get().1
    }
}

impl LatencyCore {
    pub fn link_idep(&mut self, rs: &Rc<ReservationStation>, idep_num: usize, idep_name: u8) {
        if idep_name == 0 {
            // No input dependence in this slot
            rs.set_idep_ready(idep_num);
            return;
        }

        let entry = &self.register_creators[idep_name as usize];

        match &entry.creator {
            None => rs.set_idep_ready(idep_num),
            Some(creator) => {
                rs.set_has_idep(idep_num);
                creator.add_odep(DependencyLinker {
                    consumer: Some(rs.clone()),
                    tag: Some(rs.tag.get()),
                    opnum: idep_num,
                });
            }
        }
    }

    pub fn link_odep(&mut self, rs: &Rc<ReservationStation>, odep_name: u8) {
        if odep_name == 0 {
            // No output dependence in this slot
            rs.set_no_odep();
            return;
        }

        rs.set_odep_name(odep_name);
        self.register_creators[odep_name as usize] = RegisterCreator {
            creator: Some(rs.clone()),
            output_dep: odep_name,
        };
    }
}

/// Implement Display trait for Reservation Station
impl std::fmt::Display for ReservationStation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "**Reservation Station State**")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RegisterCreator {
    pub creator: Option<Rc<ReservationStation>>,
    pub output_dep: u8,
}

impl RegisterCreator {
    pub fn null() -> Self {
        Self {
            creator: None,
            output_dep: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DependencyLinker {
    pub consumer: Option<Rc<ReservationStation>>,
    pub tag: Option<u32>,
    pub opnum: usize,
}

impl DependencyLinker {
    pub fn null() -> Self {
        Self {
            consumer: None,
            tag: None,
            opnum: 0,
        }
    }

    pub fn from_tag(rs: &Rc<ReservationStation>) -> Self {
        Self {
            consumer: Some(rs.clone()),
            tag: Some(rs.tag.get()),
            opnum: 0,
        }
    }

    pub fn valid(&self) -> bool {
        if let Some(rs) = &self.consumer {
            if self.tag.is_some() && rs.tag.get() == self.tag.unwrap() {
                return true;
            }
        }
        false
    }

    pub fn ideps_ready(&self) -> bool {
        if let Some(rsq) = &self.consumer {
            rsq.ideps_ready()
        } else {
            false
        }
    }
}
