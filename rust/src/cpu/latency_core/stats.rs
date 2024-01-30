use crate::cpu::stats::{InstructionStats, MemoryStats, StatCounter};

pub struct ProcessorStats {
    pub committed_instrs: StatCounter,
    pub committed_loads: StatCounter,
    pub committed_stores: StatCounter,
    pub committed_memory: StatCounter,
    pub committed_branches: StatCounter,

    pub executed_instrs: StatCounter,
    pub executed_loads: StatCounter,
    pub executed_stores: StatCounter,
    pub executed_memory: StatCounter,
    pub executed_branches: StatCounter,

    pub ipc_executed: f64,
    pub ipc_committed: f64,
    pub cpi_committed: f64,

    pub instr: InstructionStats,
    pub mem: MemoryStats,
}

impl ProcessorStats {
    pub fn new() -> Self {
        Self {
            committed_instrs: 0,
            committed_loads: 0,
            committed_stores: 0,
            committed_memory: 0,
            committed_branches: 0,

            executed_instrs: 0,
            executed_loads: 0,
            executed_stores: 0,
            executed_memory: 0,
            executed_branches: 0,

            ipc_executed: 0.0,
            ipc_committed: 0.0,
            cpi_committed: 0.0,

            instr: InstructionStats::new(),
            mem: MemoryStats::new(),
        }
    }

    pub fn update_summary(&mut self, cycles: u64) {
        self.committed_memory = self.committed_loads + self.committed_stores;
        self.executed_memory = self.executed_loads + self.executed_stores;

        self.ipc_executed = self.executed_instrs as f64 / cycles as f64;
        self.ipc_committed = self.committed_instrs as f64 / cycles as f64;
        self.cpi_committed = cycles as f64 / self.committed_instrs as f64;
    }
}
