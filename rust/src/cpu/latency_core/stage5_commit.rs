use super::LatencyCore;
use crate::cpu::{exception::Exception, latency_core::instr::Opcode};

impl LatencyCore {
    pub fn run_commit_stage(&mut self) -> Result<(), Exception> {
        let mut commited_cnt = 0;
        while !self.rsq.is_empty() && commited_cnt < self.params.commit_width {
            let rs = self.rsq.front().unwrap();

            if !rs.completed.get() {
                break;
            }

            if rs.ea_comp.get() {
                assert!(!self.lsq.is_empty());
                let lsq_entry = self.lsq.front().unwrap();
                if !lsq_entry.completed.get() {
                    break;
                }
                if lsq_entry.op.get() == Opcode::Store {
                    if let Some(fu) = self.functional_unit_pool.get_exec_unit(crate::cpu::latency_core::components::functional_unit_pool::ExecutionClass::Store) {
                        fu.set_busy();
                    } else {
                        break;
                    }
                }
                self.lsq.pop_front();
            }
            self.rsq.pop_front();
            commited_cnt += 1;
        }
        Ok(())
    }
}
