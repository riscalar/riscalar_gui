use super::{components::cache::CacheOp, instr::instr_opcode, LatencyCore};
use crate::cpu::exception::Exception;

#[derive(Debug, Clone, Copy)]
pub struct FetchItem {
    pub instr: u64,
    pub current_pc: u64,
    pub pred_pc: u64,
}

impl LatencyCore {
    pub fn run_fetch_stage(&mut self) -> Result<(), Exception> {
        let mut i = 0;

        // TODO: Used when branch prediction is implemented, see comments
        let mut num_branches = 0;
        let mut done = false;

        while i < (self.params.fetch_speed * self.params.decode_width)
            && self.fetch_dispatch_queue.len() < self.params.fetch_queue_size as usize
            && !done
        {
            // Update fetch_pc
            self.fetch_pc = self.fetch_pred_pc;

            // Fetch instruction
            let instr = self.fetch()?;

            // Calculate latency to access said instruction
            // Best case latency is that instruction is in il1
            if !self.fetch_stall {
                let latency = self.cache_hierachy.access(
                    &self.mem_latency,
                    self.fetch_pc,
                    4,
                    CacheOp::Read,
                    true,
                    self.tick,
                );

                // If condition is not met, means that access I-cache has missed
                if latency > 1 {
                    // So fetch stage needs to stall until it is resolved
                    self.fetch_stall_cycles += latency - 1;
                    self.fetch_stall = true;
                    break;
                }
            }

            self.fetch_stall = false;

            // We now have a valid instruction.
            // Need to check the predictor to see what the predicted PC is after this instr before writing to the fetch -> dispatch queue
            //
            if let Some(pred) = &self.branch_predictor {
                let opcode = instr_opcode(&instr)?;
                if opcode.is_ctrl() {
                    self.fetch_pred_pc = pred.lookup(&self.fetch_pc, &0);
                } else {
                    self.fetch_pred_pc = 0
                }

                match self.fetch_pred_pc {
                    0 => self.fetch_pred_pc = self.fetch_pc + 4,
                    _ => {
                        num_branches += 1;
                        if num_branches >= self.params.fetch_speed {
                            done = true;
                        }
                    }
                }
            } else {
                //*/
                self.fetch_pred_pc = self.fetch_pc + 4;
            }

            // Commit instruction to the fetch-dispatch queue
            self.fetch_dispatch_queue.push_back(FetchItem {
                instr,
                current_pc: self.fetch_pc,
                pred_pc: self.fetch_pred_pc,
            });

            i += 1;
        }
        Ok(())
    }
}
