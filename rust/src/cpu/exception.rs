use std::fmt::Display;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction(u64),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StoreAMOPageFault(u64),
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exception::InstructionAddressMisaligned => write!(f, "InstructionAddressMisaligned"),
            Exception::InstructionAccessFault => write!(f, "InstructionAccessFault"),
            Exception::IllegalInstruction(inst) => write!(f, "IllegalInstruction: 0x{:x}", inst),
            Exception::Breakpoint => write!(f, "Breakpoint"),
            Exception::LoadAddressMisaligned => write!(f, "LoadAddressMisaligned"),
            Exception::LoadAccessFault => write!(f, "LoadAccessFault"),
            Exception::StoreAMOAddressMisaligned => write!(f, "StoreAMOAddressMisaligned"),
            Exception::StoreAMOAccessFault => write!(f, "StoreAMOAccessFault"),
            Exception::EnvironmentCallFromUMode => write!(f, "EnvironmentCallFromUMode"),
            Exception::EnvironmentCallFromSMode => write!(f, "EnvironmentCallFromSMode"),
            Exception::EnvironmentCallFromMMode => write!(f, "EnvironmentCallFromMMode"),
            Exception::InstructionPageFault(addr) => {
                write!(f, "InstructionPageFault: 0x{:x}", addr)
            }
            Exception::LoadPageFault(addr) => write!(f, "LoadPageFault: 0x{:x}", addr),
            Exception::StoreAMOPageFault(addr) => write!(f, "StoreAMOPageFault: 0x{:x}", addr),
        }
    }
}

impl Exception {
    // fn code(&self) -> u64 {
    //     match self {
    //         Exception::InstructionAddressMisaligned => 0,
    //         Exception::InstructionAccessFault => 1,
    //         Exception::IllegalInstruction(_) => 2,
    //         Exception::Breakpoint => 3,
    //         Exception::LoadAddressMisaligned => 4,
    //         Exception::LoadAccessFault => 5,
    //         Exception::StoreAMOAddressMisaligned => 6,
    //         Exception::StoreAMOAccessFault => 7,
    //         Exception::EnvironmentCallFromUMode => 8,
    //         Exception::EnvironmentCallFromSMode => 9,
    //         Exception::EnvironmentCallFromMMode => 10,
    //         Exception::InstructionPageFault(_) => 11,
    //         Exception::LoadPageFault(_) => 12,
    //         Exception::StoreAMOPageFault(_) => 13,
    //     }
    // }
}

// #[derive(Debug)]
// pub enum Trap {
//     Contained,
//     Requested,
//     Invisible,
//     Fatal,
// }
