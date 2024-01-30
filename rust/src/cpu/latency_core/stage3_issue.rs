use super::{
    components::cache::CacheOp,
    instr::{instr_resource, long_latency_instr, Opcode},
    reservation_station::ReservationStation,
    LatencyCore,
};
use crate::cpu::exception::Exception;
use std::{collections::BinaryHeap, rc::Rc};

impl LatencyCore {
    pub fn run_issue_stage(&mut self) -> Result<(), Exception> {
        // let mut temp_heap = ExecutionReadyQueue::with_capacity(self.ready_queue.len());
        // std::mem::swap(&mut self.ready_queue, &mut temp_heap);
        let mut temp_heap = self.ready_queue.clone();
        self.ready_queue.clear();

        let mut issued_cnt = 0;
        while let Some(entry) = temp_heap.queue.pop() {
            if entry.valid() {
                if issued_cnt < self.params.issue_width {
                    entry.rs.unqueue();

                    if entry.rs.in_lsq && entry.rs.op.get() == Opcode::Store {
                        entry.rs.set_issued();
                        entry.rs.set_completed();

                        debug_assert!(entry.rs.odep_name.get() == 0);
                        assert!(!entry.rs.recover_instr.get());

                        issued_cnt += 1;
                    } else if let Some(resource) =
                        instr_resource(entry.rs.op.get(), entry.rs.instr)?
                    {
                        if let Some(fu) = self.functional_unit_pool.get_exec_unit(resource) {
                            entry.rs.set_issued();
                            fu.set_busy();
                            if entry.rs.in_lsq && entry.rs.op.get() == Opcode::Load {
                                let mut load_latency = 0;

                                for lsq_entry in self.lsq.iter() {
                                    if lsq_entry.op.get() == Opcode::Store
                                        && lsq_entry.addr == entry.rs.addr
                                    {
                                        load_latency = 1;
                                        break;
                                    }
                                }

                                if load_latency == 0 {
                                    load_latency = self.cache_hierachy.access(
                                        &self.mem_latency,
                                        entry.rs.addr,
                                        4,
                                        CacheOp::Read,
                                        false,
                                        self.tick,
                                    );
                                }
                                self.event_queue
                                    .add(entry.rs.clone(), self.tick + load_latency);
                            } else {
                                self.event_queue
                                    .add(entry.rs.clone(), self.tick + fu.op_lat());
                            }
                            issued_cnt += 1;
                        } else {
                            self.ready_queue.add(entry.rs.clone());
                        }
                    } else {
                        entry.rs.set_issued();

                        self.event_queue.add(entry.rs.clone(), self.tick + 1);
                        issued_cnt += 1;
                    }
                } else if entry.valid() {
                    entry.rs.unqueue();
                    self.ready_queue.add(entry.rs.clone());
                }
            }
        }
        Ok(())
    }

    pub fn refresh_lsq(&mut self) -> Result<(), Exception> {
        let mut unknown_store_targets: Vec<u64> = Vec::new();

        for entry in self.lsq.iter_mut() {
            match entry.op.get() {
                Opcode::Store => {
                    if !entry.store_addr_ready() {
                        break;
                    } else if !entry.store_op_ready() {
                        unknown_store_targets.push(entry.addr);
                    } else {
                        unknown_store_targets.retain(|&x| x != entry.addr);
                    }
                }
                Opcode::Load => {
                    if !entry.queued.get()
                        && !entry.issued.get()
                        && !entry.completed.get()
                        && entry.ideps_ready()
                    {
                        if unknown_store_targets.contains(&entry.addr) {
                            break;
                        } else {
                            self.ready_queue.add(entry.clone());
                        }
                    }
                }
                _ => panic!("LSQ entry that is not a load or a store found when refreshing LSQ"),
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ExecutionReadyQueue {
    pub queue: BinaryHeap<ExecutionReadyEntry>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ExecutionReadyEntry {
    pub rs: Rc<ReservationStation>,
    pub tag: u32,
    pub seq: u64,
    pub high_priority: bool,
}

impl ExecutionReadyEntry {
    pub fn valid(&self) -> bool {
        self.rs.tag.get() == self.tag
    }
}

impl Ord for ExecutionReadyEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // if long latency, should be executed first
        self.high_priority
            .cmp(&other.high_priority)
            // if both are long latency, compare seq number
            .then_with(|| self.seq.cmp(&other.seq).reverse())
    }
}

impl PartialOrd for ExecutionReadyEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ExecutionReadyQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn add(&mut self, entry: Rc<ReservationStation>) {
        if entry.queued.get() {
            panic!("Tried to add a queued entry to the ready queue")
        }
        entry.set_queued();

        let link = ExecutionReadyEntry {
            rs: entry.clone(),
            tag: entry.tag.get(),
            seq: entry.seq,
            high_priority: entry.in_lsq
                || entry.op.get().is_ctrl()
                || long_latency_instr(entry.instr),
        };

        self.queue.push(link);
    }
}
