use super::{instr::*, reservation_station::ReservationStation, LatencyCore};
use crate::cpu::{exception::Exception, latency_core::reservation_station::DependencyLinker};
use std::rc::Rc;

impl LatencyCore {
    pub fn run_dispatch_stage(&mut self) -> Result<(), Exception> {
        let mut dispatched_cnt = 0;
        let mut fetch_redirected = false;
        while
        /* Still space to decode instructions */
        dispatched_cnt < (self.params.fetch_speed * self.params.decode_width)
        /* and rsq is not full */
        && self.rsq.len() < self.params.rsq_size as usize
        /* and LSQ is not full */
        && self.lsq.len() < self.params.lsq_size as usize
        /* and instructions are still available from the fetch unit */
        && !self.fetch_dispatch_queue.is_empty()
        /* and not executing on a mis-predicted speculative path*/
         && !(self.params.issue_no_misspec && self.spec_mode)
        {
            if self.params.issue_inorder
                && self.last_operation.valid()
                && !self.last_operation.ideps_ready()
            {
                break;
            }
            let fetch_head = self.fetch_dispatch_queue.front().unwrap();

            self.pc = fetch_head.current_pc;
            self.pred_pc = fetch_head.pred_pc;
            self.next_pc = self.pc + 4;

            let instr = fetch_head.instr;
            let instr_opcode = instr_opcode(&instr)?;

            // Drain scheduler for system instructions
            if instr_opcode == Opcode::System {
                if !self.rsq.is_empty() {
                    break;
                } else if self.spec_mode {
                    panic!("System instruction executing while in speculative mode");
                }
            }

            let (idep_names, odep_name) = instr_dep(instr)?;
            let (target_pc, addr) = self.execute(instr)?;

            // match instr_opcode {
            //     Opcode::Load => {
            //         self.stats.total_refs += 1;
            //         self.stats.total_loads += 1;
            //         if !self.spec_mode {
            //             self.stats.num_refs += 1;
            //             self.stats.num_loads += 1;
            //         }
            //     }
            //     Opcode::Store => {
            //         self.stats.total_refs += 1;
            //         if !self.spec_mode {
            //             self.stats.num_refs += 1;
            //         }
            //     }
            //     _ => (),
            // }

            let branch_pred_taken = self.pred_pc != self.next_pc;
            let branch_pred_correct = instr_opcode.is_ctrl() && target_pc.unwrap() == self.pred_pc;

            if branch_pred_taken && (self.pred_perfect || !branch_pred_correct) {
                self.fetch_pc = self.next_pc;
                self.fetch_pred_pc = self.next_pc;

                if self.pred_perfect {
                    self.pred_pc = self.next_pc;
                }

                self.fetch_dispatch_queue.truncate(1);

                if !self.pred_perfect {
                    self.fetch_stall_cycles = self.params.fetch_branch_penalty;
                }

                fetch_redirected = true;
            }

            if !instr_opcode.is_noop() {
                self.inst_seq += 1;
                let rs = Rc::new(ReservationStation::new_rsq_entry(
                    self.tick - 1,
                    instr,
                    self.pc,
                    self.next_pc,
                    self.pred_pc,
                    self.spec_mode,
                    (self.rsq.len() + 1) as u32,
                    self.inst_seq,
                ));

                if instr_opcode.is_mem() {
                    rs.addr_comput_op();

                    self.inst_seq += 1;
                    let lsq_rs = Rc::new(ReservationStation::new_lsq_entry(
                        self.tick - 1,
                        instr,
                        self.pc,
                        self.next_pc,
                        self.pred_pc,
                        self.spec_mode,
                        (self.lsq.len() + 1) as u32,
                        self.inst_seq,
                        addr.unwrap(),
                    ));

                    self.link_idep(&rs, 0, 0);
                    self.link_idep(&rs, 1, idep_names.1);
                    self.link_odep(&rs, 32);

                    self.link_idep(&lsq_rs, 0, idep_names.0);
                    self.link_idep(&lsq_rs, 1, 32);
                    self.link_odep(&lsq_rs, odep_name);

                    self.rsq.push_back(rs.clone());
                    self.lsq.push_back(lsq_rs.clone());

                    if rs.ideps_ready() {
                        self.ready_queue.add(rs);
                    }
                    self.last_operation = DependencyLinker::from_tag(&lsq_rs);

                    if lsq_rs.ideps_ready() && instr_opcode == Opcode::Store {
                        self.ready_queue.add(lsq_rs);
                    }
                } else {
                    self.link_idep(&rs, 0, idep_names.0);
                    self.link_idep(&rs, 1, idep_names.1);
                    self.link_odep(&rs, odep_name);

                    self.rsq.push_back(rs.clone());
                    dispatched_cnt += 1;

                    if rs.ideps_ready() {
                        self.ready_queue.add(rs);
                        self.last_operation = DependencyLinker::null()
                    } else {
                        self.last_operation = DependencyLinker::from_tag(&rs);
                    }
                }
            }
            /* one more instruction executed, speculative or otherwise */
            // sim_total_insn++;
            // if (MD_OP_FLAGS(op) & F_CTRL)
            //   sim_total_branches++;

            if !self.spec_mode && self.pred_pc != self.next_pc && !fetch_redirected {
                self.spec_mode = true;
                self.recover_pc = self.next_pc;
                if instr_opcode != Opcode::NoOp {
                    self.rsq.back_mut().unwrap().set_recover_instr();
                }
            }

            assert!(self.fetch_dispatch_queue.pop_front().is_some());
        }
        Ok(())
    }
}
