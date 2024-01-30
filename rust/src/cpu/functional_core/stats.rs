use crate::cpu::stats::{InstructionStats, MemoryStats};

use super::FunctionalCore;
use serde_json::{json, Value};

impl FunctionalCore {
    pub fn get_sim_res_json(&self) -> Value {
        json!({
           "stats":{
               "top_level": self.get_top_level_stats(),
               "instr": self.get_instr_stats(),
               "mem": self.get_mem_stats(),
            },
           "reg_state": self.get_reg_state(),
        })
    }

    fn get_top_level_stats(&self) -> Value {
        json!({
           "cycles": self.tick,
           "total_instr_executed": self.stats.instr.sum(),
           "total_mem_accesses" : self.stats.mem.sum(),
        })
    }

    fn get_instr_stats(&self) -> Value {
        serde_json::to_value(self.stats.instr).unwrap()
    }

    fn get_mem_stats(&self) -> Value {
        serde_json::to_value(self.stats.mem).unwrap()
    }

    fn get_reg_state(&self) -> Value {
        serde_json::to_value(self.xreg.clone()).unwrap()
    }
}

pub struct ProcessorStats {
    pub instr: InstructionStats,
    pub mem: MemoryStats,
}

impl ProcessorStats {
    pub fn new() -> Self {
        Self {
            instr: InstructionStats::new(),
            mem: MemoryStats::new(),
        }
    }
}
