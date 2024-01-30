use core::fmt;
use serde::Serialize;
pub type StatCounter = u128;
pub type Tick = u64;

#[derive(Serialize, Clone, Copy)]
pub struct InstructionStats {
    pub instr_load: StatCounter,
    pub instr_store: StatCounter,
    pub instr_jump: StatCounter,
    pub instr_branch: StatCounter,
    pub instr_arith: StatCounter,
    pub instr_fence: StatCounter,
}

impl InstructionStats {
    pub fn new() -> Self {
        Self {
            instr_load: 0,
            instr_store: 0,
            instr_jump: 0,
            instr_branch: 0,
            instr_arith: 0,
            instr_fence: 0,
        }
    }

    pub fn sum(&self) -> StatCounter {
        self.instr_load
            + self.instr_store
            + self.instr_jump
            + self.instr_branch
            + self.instr_arith
            + self.instr_fence
    }
}

impl fmt::Display for InstructionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Load instructions:             {}\n\
             Store instructions:            {}\n\
             Jump instructions:             {}\n\
             Branch instructions:           {}\n\
             Arithmetic instructions:       {}\n\
             Fence instructions:            {}\n",
            self.instr_load,
            self.instr_store,
            self.instr_jump,
            self.instr_branch,
            self.instr_arith,
            self.instr_fence,
        )
    }
}

#[derive(Serialize, Clone, Copy)]
pub struct MemoryStats {
    pub mem_inst_reads: StatCounter,
    pub mem_data_reads: StatCounter,
    pub mem_data_writes: StatCounter,
}

impl MemoryStats {
    pub fn new() -> Self {
        Self {
            mem_inst_reads: 0,
            mem_data_reads: 0,
            mem_data_writes: 0,
        }
    }

    pub fn sum(&self) -> StatCounter {
        self.mem_inst_reads + self.mem_data_reads + self.mem_data_writes
    }
}
