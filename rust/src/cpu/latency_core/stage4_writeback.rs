use super::{
    reservation_station::{RegisterCreator, ReservationStation},
    LatencyCore,
};
use crate::cpu::{exception::Exception, latency_core::instr::Opcode, stats::Tick};
use std::{collections::BinaryHeap, rc::Rc};

impl LatencyCore {
    pub fn run_writeback_stage(&mut self) -> Result<(), Exception> {
        while let Some(rs) = self.event_queue.pop(self.tick) {
            rs.set_completed();

            if rs.recover_instr.get() {
                assert!(!rs.in_lsq);
                self.reservation_station_queue_recover(&rs);
                self.reset_speculative_state();
                self.fetch_stall_cycles = self.params.fetch_branch_penalty;
            }

            if rs.odep_name.get() != 0 {
                match rs.spec_mode {
                    true => {
                        let creator = &self.spec_register_creators[rs.odep_name.get() as usize];
                        if let Some(rs_create) = &creator.creator {
                            if Rc::ptr_eq(rs_create, &rs) {
                                self.spec_register_creators[rs.odep_name.get() as usize] =
                                    RegisterCreator::null();
                            }
                        }
                    }
                    false => {
                        let creator = &self.register_creators[rs.odep_name.get() as usize];
                        if let Some(rs_create) = &creator.creator {
                            if Rc::ptr_eq(rs_create, &rs) {
                                self.register_creators[rs.odep_name.get() as usize] =
                                    RegisterCreator::null();
                            }
                        }
                    }
                }

                /* walk output list, queue up ready operations */
                for olink in rs.odep_list.borrow_mut().iter() {
                    if olink.valid() {
                        let consumer = olink.consumer.as_ref().unwrap();
                        assert!(!consumer.idep_ready(olink.opnum));

                        consumer.set_idep_ready(olink.opnum);
                        if consumer.ideps_ready()
                            && (!consumer.in_lsq || consumer.op.get() == Opcode::Store)
                        {
                            self.ready_queue.add(consumer.clone());
                        }
                    }
                }
                rs.odep_list.borrow_mut().clear();
            }
        }
        Ok(())
    }

    fn reset_speculative_state(&mut self) {
        self.spec_mode = false;

        self.spec_xreg_valid.fill(false);
        self.spec_mem_clear();
    }

    fn reservation_station_queue_recover(&mut self, branch_rs: &Rc<ReservationStation>) {
        while let Some(rs) = self.rsq.back() {
            if Rc::ptr_eq(rs, branch_rs) {
                break;
            }

            if rs.ea_comp.get() {
                let lsq_entry = self.lsq.pop_back().unwrap();
                lsq_entry.invalidate();
            }

            rs.invalidate();

            self.rsq.pop_back();
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventQueue {
    queue: BinaryHeap<EventQueueEntry>,
}

impl EventQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, entry: Rc<ReservationStation>, tick: Tick) {
        assert!(!entry.completed.get());

        self.queue.push(EventQueueEntry {
            rs: entry.clone(),
            tag: entry.tag.get(),
            when: tick,
        });
    }

    pub fn pop(&mut self, tick: Tick) -> Option<Rc<ReservationStation>> {
        if let Some(event) = self.queue.peek() {
            if event.when <= tick {
                match event.valid() {
                    true => return Some(self.queue.pop().unwrap().rs),
                    false => return self.pop(tick),
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EventQueueEntry {
    pub rs: Rc<ReservationStation>,
    pub tag: u32,
    pub when: Tick,
}

impl EventQueueEntry {
    pub fn valid(&self) -> bool {
        self.rs.tag.get() == self.tag
    }
}

impl Ord for EventQueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.when.cmp(&other.when).reverse()
    }
}

impl PartialOrd for EventQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
