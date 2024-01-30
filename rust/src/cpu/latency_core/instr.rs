use super::components::functional_unit_pool::ExecutionClass;
use crate::cpu::{exception::Exception, instr::*};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Opcode {
    Lui,
    AuiPc,
    Jal,
    JalR,
    Branch,
    Load,
    Store,
    Imm,
    Imm32,
    Op,
    Op32,
    MiscMem,
    System,
    NoOp,
}

impl Opcode {
    pub fn is_ctrl(&self) -> bool {
        matches!(self, Opcode::Jal | Opcode::JalR | Opcode::Branch)
    }

    pub fn is_noop(&self) -> bool {
        matches!(self, Opcode::NoOp)
    }

    pub fn is_mem(&self) -> bool {
        matches!(self, Opcode::Load | Opcode::Store)
    }
}

// impl Instr {
//     pub fn decode(instr: u64) -> Result<Self, Exception> {
//         let opcode = match instr & 0x7f {
//             OPCODE_LUI => Opcode::Lui,
//             OPCODE_AUIPC => Opcode::AuiPc,
//             OPCODE_JAL => Opcode::Jal,
//             OPCODE_JALR => Opcode::JalR,
//             OPCODE_BRANCH => Opcode::Branch,
//             OPCODE_LOAD => Opcode::Load,
//             OPCODE_STORE => Opcode::Store,
//             OPCODE_OP_IMM => Opcode::Imm,
//             OPCODE_OP_IMM32 => Opcode::Imm32,
//             OPCODE_OP => Opcode::Op,
//             OPCODE_OP32 => Opcode::Op,
//             OPCODE_MISC_MEM => Opcode::MiscMem,
//             OPCODE_SYSTEM => Opcode::System,
//             _ => return Err(Exception::IllegalInstruction(instr)),
//         };
//         Ok(Self {
//             opcode,
//             rd: ((instr & 0x00000F80) >> 7) as u8,
//             rs1: ((instr & 0x000F8000) >> 15) as u8,
//             rs2: ((instr & 0x01F00000) >> 20) as u8,
//             funct3: (instr & 0x00007000) >> 12,
//             funct7: (instr & 0xFE000000) >> 25,
//             imm20: (instr & 0xFFFFF000) as i32 as i64 as u64,
//             imm12: ((instr as i32 as i64) >> 20) as u64,
//         })
//     }
// }

pub fn instr_opcode(instr: &u64) -> Result<Opcode, Exception> {
    if *instr == 0x00000013 {
        return Ok(Opcode::NoOp);
    }

    match instr & 0x7f {
        OPCODE_LUI => Ok(Opcode::Lui),
        OPCODE_AUIPC => Ok(Opcode::AuiPc),
        OPCODE_JAL => Ok(Opcode::Jal),
        OPCODE_JALR => Ok(Opcode::JalR),
        OPCODE_BRANCH => Ok(Opcode::Branch),
        OPCODE_LOAD => Ok(Opcode::Load),
        OPCODE_STORE => Ok(Opcode::Store),
        OPCODE_OP_IMM => Ok(Opcode::Imm),
        OPCODE_OP_IMM32 => Ok(Opcode::Imm32),
        OPCODE_OP => Ok(Opcode::Op),
        OPCODE_OP32 => Ok(Opcode::Op32),
        OPCODE_MISC_MEM => Ok(Opcode::MiscMem),
        OPCODE_SYSTEM => Ok(Opcode::System),
        _ => Err(Exception::IllegalInstruction(*instr)),
    }
}

pub fn instr_dep(instr: u64) -> Result<((u8, u8), u8), Exception> {
    let opcode = instr_opcode(&instr)?;
    let rs1 = ((instr & 0x000F8000) >> 15) as u8;
    let rs2 = ((instr & 0x01F00000) >> 20) as u8;
    let rd = ((instr & 0x00000F80) >> 7) as u8;

    match opcode {
        Opcode::Lui | Opcode::AuiPc => Ok(((0, 0), rd)),
        Opcode::Jal | Opcode::JalR => Ok(((0, 0), rd)),
        Opcode::Branch => Ok(((rs1, rs2), 0)),
        Opcode::Load => Ok(((0, rs1), rd)),
        Opcode::Store => Ok(((rs2, rs1), 0)),
        Opcode::Imm | Opcode::Imm32 => Ok(((rs1, 0), rd)),
        Opcode::Op | Opcode::Op32 => Ok(((rs1, rs2), rd)),
        Opcode::MiscMem | Opcode::System | Opcode::NoOp => Ok(((0, 0), 0)),
    }
}

pub fn instr_resource(opcode: Opcode, instr: u64) -> Result<Option<ExecutionClass>, Exception> {
    match opcode {
        Opcode::Lui | Opcode::AuiPc | Opcode::Jal | Opcode::JalR | Opcode::Branch => {
            Ok(Some(ExecutionClass::IntALU))
        }
        Opcode::Load => Ok(Some(ExecutionClass::Load)),
        Opcode::Store => Ok(Some(ExecutionClass::Store)),
        Opcode::Imm => Ok(Some(ExecutionClass::IntALU)),
        Opcode::Imm32 => Ok(Some(ExecutionClass::IntALU)),
        Opcode::Op => match ((instr & 0x00007000) >> 12, (instr & 0xFE000000) >> 25) {
            // mul, mulh, mulhsu, mulhu
            (0x0, 0x01) | (0x1, 0x01) => Ok(Some(ExecutionClass::IntMult)),
            (0x2, 0x01) | (0x3, 0x01) => Ok(Some(ExecutionClass::IntMult)),
            // div, divu, rem, remu
            (0x4, 0x01) | (0x5, 0x01) => Ok(Some(ExecutionClass::IntDiv)),
            (0x7, 0x01) | (0x6, 0x01) => Ok(Some(ExecutionClass::IntDiv)),
            _ => Ok(Some(ExecutionClass::IntALU)),
        },
        Opcode::Op32 => match ((instr & 0x00007000) >> 12, (instr & 0xFE000000) >> 25) {
            // mulw,
            (0x0, 0x01) => Ok(Some(ExecutionClass::IntMult)),
            // divw, divuw, remw, remuw
            (0x4, 0x01) | (0x5, 0x01) => Ok(Some(ExecutionClass::IntDiv)),
            (0x7, 0x01) | (0x6, 0x01) => Ok(Some(ExecutionClass::IntDiv)),
            _ => Ok(Some(ExecutionClass::IntALU)),
        },
        Opcode::MiscMem | Opcode::System | Opcode::NoOp => Ok(None),
    }
}

pub fn long_latency_instr(instr: u64) -> bool {
    let opcode = instr_opcode(&instr).expect("Found a bad instruction while checking if an instruction from a Reservation Station has a long latency");
    let funct3 = (instr & 0x00007000) >> 12;
    let funct7 = (instr & 0xFE000000) >> 25;

    match opcode {
        Opcode::Lui | Opcode::AuiPc | Opcode::Jal | Opcode::JalR | Opcode::Branch => false,
        Opcode::Load => true,
        Opcode::Store => true,
        Opcode::Imm => false,
        Opcode::Imm32 => false,
        Opcode::Op => match (funct3, funct7) {
            // mul, mulh, mulhsu, mulhu
            (0x0, 0x01) | (0x1, 0x01) => true,
            (0x2, 0x01) | (0x3, 0x01) => true,
            // div, divu, rem, remu
            (0x4, 0x01) | (0x5, 0x01) => true,
            (0x7, 0x01) | (0x6, 0x01) => true,
            _ => false,
        },
        Opcode::Op32 => match (funct3, funct7) {
            // mulw,
            (0x0, 0x01) => true,
            // divw, divuw, remw, remuw
            (0x4, 0x01) | (0x5, 0x01) => true,
            (0x7, 0x01) | (0x6, 0x01) => true,
            _ => false,
        },
        Opcode::MiscMem | Opcode::System | Opcode::NoOp => false,
    }
}
