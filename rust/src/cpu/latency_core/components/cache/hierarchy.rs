use super::{Cache, CacheHierachy, CacheOp, CacheOptions, MemLatency};
use crate::cpu::{latency_core::latency_args::CacheConfig, stats::Tick};

impl CacheHierachy {
    pub fn access(
        &mut self,
        mem: &MemLatency,
        address: u64,
        size: u64,
        operation: CacheOp,
        instr: bool,
        now: Tick,
    ) -> u64 {
        let mut lat: u64 = 0;

        if instr {
            if let Some(l1) = self.cache_il1.as_mut().or(self.unified_l1.as_mut()) {
                match l1.access(address, size, operation, now) {
                    // L1 hit
                    (true, l1_hit_lat, _, _) => lat += l1_hit_lat,
                    (false, l1_miss_lat, l1_eaddr, l1_eidx) => {
                        lat += l1_miss_lat;

                        if let Some(l2) = self.cache_il2.as_mut().or(self.unified_l2.as_mut()) {
                            if let Some(l1_eaddr) = l1_eaddr {
                                match l2.access(l1_eaddr, l1.block_size, CacheOp::Write, now + lat)
                                {
                                    (true, l2_evict_hit_lat, _, _) => lat += l2_evict_hit_lat,
                                    (false, l2_emiss_lat, l2_eaddr, l2_eidx) => {
                                        lat += l2_emiss_lat;
                                        if l2_eaddr.is_some() {
                                            lat += mem.access(l2.block_size, CacheOp::Write);
                                        }
                                        lat += mem.access(l2.block_size, CacheOp::Read);
                                        l2.insert_replacement(
                                            l1_eaddr,
                                            now + lat,
                                            CacheOp::Write,
                                            l2_eidx,
                                        );
                                    }
                                }
                            }

                            match l2.access(address, l1.block_size, CacheOp::Read, now + lat) {
                                (true, l2_hit_lat, _, _) => lat += l2_hit_lat,
                                (false, l2_miss_lat, l2_eaddr, l2_eidx) => {
                                    lat += l2_miss_lat;
                                    if l2_eaddr.is_some() {
                                        lat += mem.access(l2.block_size, CacheOp::Write);
                                    }
                                    lat += mem.access(l2.block_size, CacheOp::Read);
                                    l2.insert_replacement(
                                        address,
                                        now + lat,
                                        CacheOp::Read,
                                        l2_eidx,
                                    );
                                }
                            }
                        } else {
                            // Access main memory if no L2
                            if l1_eaddr.is_some() {
                                lat += mem.access(l1.block_size, CacheOp::Write);
                            }
                            lat += mem.access(l1.block_size, CacheOp::Read);
                        }
                        l1.insert_replacement(address, now + lat, operation, l1_eidx);
                    }
                }
            } else {
                lat += mem.access(size, operation);
            }
        } else if !instr {
            if let Some(l1) = self.cache_dl1.as_mut().or(self.unified_l1.as_mut()) {
                match l1.access(address, size, operation, now) {
                    // L1 hit
                    (true, l1_hit_lat, _, _) => lat += l1_hit_lat,
                    (false, l1_miss_lat, l1_eaddr, l1_eidx) => {
                        lat += l1_miss_lat;

                        if let Some(l2) = self.cache_dl2.as_mut().or(self.unified_l2.as_mut()) {
                            if let Some(l1_eaddr) = l1_eaddr {
                                match l2.access(l1_eaddr, l1.block_size, CacheOp::Write, now + lat)
                                {
                                    (true, l2_evict_hit_lat, _, _) => lat += l2_evict_hit_lat,
                                    (false, l2_emiss_lat, l2_eaddr, l2_eidx) => {
                                        lat += l2_emiss_lat;
                                        if l2_eaddr.is_some() {
                                            lat += mem.access(l2.block_size, CacheOp::Write);
                                        }
                                        lat += mem.access(l2.block_size, CacheOp::Read);
                                        l2.insert_replacement(
                                            l1_eaddr,
                                            now + lat,
                                            CacheOp::Write,
                                            l2_eidx,
                                        );
                                    }
                                }
                            }

                            match l2.access(address, l1.block_size, CacheOp::Read, now + lat) {
                                (true, l2_hit_lat, _, _) => lat += l2_hit_lat,
                                (false, l2_miss_lat, l2_eaddr, l2_eidx) => {
                                    lat += l2_miss_lat;
                                    if l2_eaddr.is_some() {
                                        lat += mem.access(l2.block_size, CacheOp::Write);
                                    }
                                    lat += mem.access(l2.block_size, CacheOp::Read);
                                    l2.insert_replacement(
                                        address,
                                        now + lat,
                                        CacheOp::Read,
                                        l2_eidx,
                                    );
                                }
                            }
                        } else {
                            // Access main memory if no L2
                            if l1_eaddr.is_some() {
                                lat += mem.access(l1.block_size, CacheOp::Write);
                            }
                            lat += mem.access(l1.block_size, CacheOp::Read);
                        }
                        l1.insert_replacement(address, now + lat, operation, l1_eidx);
                    }
                }
            } else {
                lat += mem.access(size, operation);
            }
        }
        lat
    }

    pub fn new(cfg: CacheConfig) -> Self {
        let cache_il1;
        let cache_il2;
        let cache_dl1;
        let cache_dl2;
        let unified_l1;
        let unified_l2;

        if !cfg.il1.exists() {
            // No L1 instruction cache means:
            //     No L2 instruction cache
            //     No unifed caches
            cache_il1 = None;
            if cfg.il2.exists() {
                panic!("L2 instruction cache can't be defined without L1 cache");
            }
            cache_il2 = None;
            unified_l1 = None;
            unified_l2 = None;

            // If L1 data cache exists, L2 data cache can exist
            // Otherwise both don't exist
            if cfg.dl1.exists() {
                assert!(
                    !cfg.dl1.unified(),
                    "No L1 instruction cache means no unified L1 cache"
                );
                cache_dl1 = Some(Cache::cfg(cfg.dl1));
                if cfg.dl2.exists() {
                    assert!(
                        !cfg.dl2.unified(),
                        "No L2 instruction cache means no unified L2 cache"
                    );
                    cache_dl2 = Some(Cache::cfg(cfg.dl2));
                } else {
                    cache_dl2 = None;
                }
            } else {
                cache_dl1 = None;
                if cfg.dl2.exists() {
                    panic!("L2 data cache can't be defined without L1 cache");
                }
                cache_dl2 = None;
            }
        } else if cfg.il1.unified() {
            assert!(
                !cfg.dl1.unified(),
                "Both L1 caches can't be defined as unified"
            );
            cache_il1 = None;
            cache_dl1 = None;
            unified_l1 = Some(Cache::cfg(cfg.dl1));

            (cache_il2, cache_dl2, unified_l2) = l2_match(&cfg.il2, &cfg.dl2);
        } else {
            // IL1 has a configuration associated with it
            if cfg.dl1.unified() {
                cache_il1 = None;
                cache_dl1 = None;
                unified_l1 = Some(Cache::cfg(cfg.il1));
                (cache_il2, cache_dl2, unified_l2) = l2_match(&cfg.il2, &cfg.dl2);
            } else {
                unified_l1 = None;
                cache_il1 = Some(Cache::cfg(cfg.il1));
                if cfg.dl1.exists() {
                    cache_dl1 = Some(Cache::cfg(cfg.dl1));
                    (cache_il2, cache_dl2, unified_l2) = l2_match(&cfg.il2, &cfg.dl2);
                } else {
                    cache_dl1 = None;
                    cache_dl2 = None;
                    if cfg.dl2.exists() {
                        panic!("L2 data cache can't be defined without L1 cache");
                    }
                    unified_l2 = None;

                    if cfg.il2.unified() {
                        panic!("L2 unifed cache cannot be defined without L1 data cache");
                    } else if cfg.il2.exists() {
                        cache_il2 = Some(Cache::cfg(cfg.il2));
                    } else {
                        cache_il2 = None;
                    }
                }
            }
        }

        Self {
            cache_il1,
            cache_il2,
            cache_dl1,
            cache_dl2,

            unified_l1,
            unified_l2,
        }
    }
}

fn l2_match(
    il2: &CacheOptions,
    dl2: &CacheOptions,
) -> (Option<Cache>, Option<Cache>, Option<Cache>) {
    match (il2, dl2) {
        (CacheOptions::None, CacheOptions::None) => (None, None, None),

        (CacheOptions::Unified, CacheOptions::Configured { .. }) => {
            (None, None, Some(Cache::cfg(*dl2)))
        }
        (CacheOptions::Configured { .. }, CacheOptions::Unified) => {
            (None, None, Some(Cache::cfg(*il2)))
        }
        (CacheOptions::Configured { .. }, CacheOptions::Configured { .. }) => {
            (Some(Cache::cfg(*il2)), Some(Cache::cfg(*dl2)), None)
        }
        (CacheOptions::None, CacheOptions::Configured { .. }) => {
            (None, Some(Cache::cfg(*dl2)), None)
        }
        (CacheOptions::Configured { .. }, CacheOptions::None) => {
            (Some(Cache::cfg(*il2)), None, None)
        }
        (CacheOptions::Unified, CacheOptions::Unified) => {
            panic!("Both L2 caches can't be defined as unified")
        }
        (CacheOptions::None, CacheOptions::Unified)
        | (CacheOptions::Unified, CacheOptions::None) => {
            panic!("L2 unified cache can't be defined without one definition");
        }
    }
}

impl CacheOptions {
    pub fn exists(&self) -> bool {
        !matches!(self, CacheOptions::None)
    }

    pub fn unified(&self) -> bool {
        matches!(self, CacheOptions::Unified)
    }
}
