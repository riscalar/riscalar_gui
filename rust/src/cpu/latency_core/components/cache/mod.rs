use rand::rngs::ThreadRng;
use std::collections::VecDeque;

use crate::cpu::latency_core::latency_args::MemoryConfig;
mod hierarchy;
mod implementation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[flutter_rust_bridge::frb(dart_metadata=("freezed"))]
pub enum CacheOptions {
    #[default]
    None,
    Unified,
    Configured {
        num_sets: u64,
        associativity: u64,
        block_size: u64,
        replacement_policy: BlockReplacement,
        latency: u64,
    },
}

pub struct CacheHierachy {
    cache_il1: Option<Cache>,
    cache_il2: Option<Cache>,
    cache_dl1: Option<Cache>,
    cache_dl2: Option<Cache>,
    unified_l1: Option<Cache>,
    unified_l2: Option<Cache>,
}

pub struct MemLatency {
    mem_latency: (u64, u64),
    mem_bus_width: u8,
}

impl MemLatency {
    pub fn new(mem_cfg: MemoryConfig) -> Self {
        // Convert memory config to memory latency
        let binding = mem_cfg.memory_latency.unwrap_or(vec![18, 2]);
        let mut iter = binding.iter();
        let mem_latency = (*iter.next().unwrap(), *iter.next().unwrap());

        Self {
            mem_latency,
            mem_bus_width: mem_cfg.memory_bus_width,
        }
    }
    fn access(&self, size: u64, operation: CacheOp) -> u64 {
        match operation {
            CacheOp::Read => {
                self.mem_latency.0
                    + self.mem_latency.1
                        * ((size + (self.mem_bus_width - 1) as u64) / self.mem_bus_width as u64)
            }
            CacheOp::Write => 0,
        }
    }
}

// Enum to represent the block replacement strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockReplacement {
    Lru,
    Fifo,
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheOp {
    Read,
    Write,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CacheBlockStatus {
    Invalid,
    Valid,
    Dirty,
}

// Struct to represent a cache block
#[derive(Clone, Copy, PartialEq, Eq)]
struct CacheBlock {
    tag: u64, // Tag of the block to identify the address
    status: CacheBlockStatus,
    ready_time: u64, // Ready time of the block
}

// Struct to represent a cache set
struct CacheSet {
    blocks: VecDeque<CacheBlock>, // Blocks in the set
}

// Struct to represent a cache
pub struct Cache {
    sets: Vec<CacheSet>,                 // Sets in the cache
    block_size: u64,                     // Block size of the cache in bytes
    associativity: u64,                  // Associativity of the cache
    block_replacement: BlockReplacement, // Block replacement strategy (LRU, FIFO, random)
    pub hit_latency: u64,                // Hit latency of the cache in cycles

    /// Calculated masks and shifts for quick access
    block_mask: u64,
    set_shift: u64,
    set_mask: u64,
    tag_shift: u64,

    /// Random number generator for random replacement
    rng: ThreadRng,
}
