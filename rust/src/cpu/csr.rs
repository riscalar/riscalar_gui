#![allow(dead_code)]

// Machine-level CSR addresses

// Machine information registers - all are MRO (machine read only)

/// Vendor ID
/// 32-bit RO register that provides JEDEC manufacture ID of core provider
/// As this is a non-commerical implementation - returns 0
const MVENDORID: u16 = 0xF11;

/// Architecture ID
/// MXLEN-bit RO register that encodes the base microarchitcture of the hart
/// returns 0 as unimplemented
const MARCHID: u16 = 0xF12;
/// Implementation ID
/// MXLEN-bit RO register that encodes the version of the processor implementation
/// returns 0 as unimplemented
const MIMPID: u16 = 0xF13;
/// Hardware thread ID.
/// MXLEN-bit RO register that encodes the version of the processor implementation
/// returns 0 as simulator is single threaded
const MHARTID: u16 = 0xF14;

// Machine trap setup - all are MRW (machine read/write).
/// Machine status register
pub const MSTATUS: u16 = 0x300;

/// ISA and extensions.
/// A WARL registers that reports which ISA and extensions are implemented by
/// the hardware thread.
/// Contains three fields:
/// 1. [61:60] MXL - XLEN - base integer length supported
/// 2. [59:26] Fixed to 0
/// 3. [25:00] Extensions - which extensions are supported
/// Writing to MXL and Extensions are supported if the hart can change supported
/// extensions and base instruction set at run time, but since that is not implemented
/// in this simulator, we treat this register as RO.
pub const MISA: u16 = 0x301;
/// Machine exception delefation register.
pub const MEDELEG: u16 = 0x302;
/// Machine interrupt delefation register.
pub const MIDELEG: u16 = 0x303;
/// Machine interrupt-enable register.
pub const MIE: u16 = 0x304;
/// Machine trap-handler base address.
pub const MTVEC: u16 = 0x305;
/// Machine counter enable.
pub const MCOUNTEREN: u16 = 0x306;

// Machine trap handling - all are MRW (machine read/write).
/// Scratch register for machine trap handlers.
pub const MSCRATCH: u16 = 0x340;
/// Machine exception program counter.
pub const MEPC: u16 = 0x341;
/// Machine trap cause.
pub const MCAUSE: u16 = 0x342;
/// Machine bad address or instruction.
pub const MTVAL: u16 = 0x343;
/// Machine interrupt pending.
pub const MIP: u16 = 0x344;
/// Machine trap instructioni (transformed)
pub const MTINST: u16 = 0x34A;
/// Machine bad guest physical address
pub const MTVAL2: u16 = 0x34B;

#[derive(Clone)]
pub struct ControlStatusRegisters {
    csrs: [u64; 4096],
}

impl ControlStatusRegisters {
    pub fn new() -> Self {
        let misa: u64 = (2 << 62) | // XLEN = 64 -> MXL[1:0] = 0b10
			            (1 << 12) | // Extensions[12] (Integer Multiply/Divide)
		                (1 << 8 ); // Extensions[8] (RV64I base ISA)
        let mut csrs = [0; 4096];
        csrs[MISA as usize] = misa;
        Self { csrs }
    }

    pub fn read(&self, addr: u16) -> u64 {
        self.csrs[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u64) {
        match addr {
            // Read only machine information registers
            MVENDORID | MARCHID | MIMPID | MHARTID => {}
            // Read only MISA
            MISA => {}
            _ => self.csrs[addr as usize] = value,
        }
    }
}
