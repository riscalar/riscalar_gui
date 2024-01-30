use super::memory::{MEM_BASE, MEM_SIZE};
use core::fmt;
use flutter_rust_bridge::frb;
use serde::{ser::SerializeStruct, Serialize, Serializer};

pub trait RegisterFile {
    fn read(&self, reg: u64) -> u64;
    fn write(&mut self, reg: u64, value: u64);
}

#[derive(Clone)]
#[frb(dart_metadata=("freezed"))]
pub struct XRegisterFile {
    pub regs: [u64; 32],
}

impl XRegisterFile {
    pub fn new() -> Self {
        let mut xregs = [0; 32];
        xregs[2] = MEM_BASE + MEM_SIZE;

        Self { regs: xregs }
    }
}

impl RegisterFile for XRegisterFile {
    fn read(&self, reg: u64) -> u64 {
        self.regs[reg as usize]
    }

    fn write(&mut self, reg: u64, value: u64) {
        match reg {
            0 => (),
            _ => self.regs[reg as usize] = value,
        }
    }
}

impl Serialize for XRegisterFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("XRegisterFile", 32)?;
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in abi.iter().enumerate() {
            state.serialize_field(i.1, &self.regs[i.0])?;
        }
        state.end()
    }
}

impl fmt::Display for XRegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        let mut output = String::from("");
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\nx{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                output,
                i,
                abi[i],
                self.read(i as u64),
                i + 1,
                abi[i + 1],
                self.read(i as u64 + 1),
                i + 2,
                abi[i + 2],
                self.read(i as u64 + 2),
                i + 3,
                abi[i + 3],
                self.read(i as u64 + 3),
            );
        }
        write!(f, "{}", output)
    }
}
