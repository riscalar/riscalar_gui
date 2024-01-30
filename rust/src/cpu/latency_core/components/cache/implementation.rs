use super::*;
use crate::cpu::stats::Tick;
use rand::Rng;

impl Cache {
    pub fn cfg(cfg: CacheOptions) -> Self {
        match cfg {
            CacheOptions::None | CacheOptions::Unified => panic!(),
            CacheOptions::Configured {
                num_sets,
                associativity,
                block_size,
                replacement_policy,
                latency,
            } => Cache::new(
                num_sets,
                associativity,
                block_size,
                replacement_policy,
                latency,
            ),
        }
    }

    // Constructor for Cache struct
    pub fn new(
        num_sets: u64,
        block_size: u64,
        associativity: u64,
        block_replacement: BlockReplacement,
        hit_latency: u64,
    ) -> Self {
        // Create empty cache sets
        let mut sets = Vec::with_capacity(num_sets as usize);
        for _ in 0..num_sets {
            sets.push(CacheSet {
                blocks: VecDeque::from(vec![
                    CacheBlock {
                        tag: 0,
                        status: CacheBlockStatus::Invalid,
                        ready_time: 0,
                    };
                    associativity as usize
                ]),
            });
        }

        // Initialize the Cache struct
        Cache {
            sets,
            block_size,
            associativity,
            block_replacement,
            hit_latency,

            /// Mask to extract block offset from address (No need to shift as block is at LSB)
            block_mask: block_size - 1,
            /// Number of bits to shift to remove offset
            set_shift: block_size.ilog2() as u64,
            /// Mask to extract set index from shifted address
            set_mask: num_sets - 1,
            /// Number of bits to shift to remove offset and set index and get tag
            tag_shift: (block_size.ilog2() + num_sets.ilog2()) as u64,

            rng: rand::thread_rng(),
        }
    }

    #[inline]
    fn cache_tag(&self, address: u64) -> u64 {
        address >> self.tag_shift
    }

    #[inline]
    fn cache_set(&self, address: u64) -> u64 {
        (address >> self.set_shift) & self.set_mask
    }

    // Function to simulate an access to the cache
    pub fn access(
        &mut self,
        address: u64,
        _size: u64,
        operation: CacheOp,
        now: Tick,
    ) -> (bool, u64, Option<u64>, usize) {
        let tag = self.cache_tag(address);
        let set = self.cache_set(address);

        let cache_set = &mut self.sets[set as usize];
        let cache_block = cache_set
            .blocks
            .iter_mut()
            .enumerate()
            .find(|(_, block)| block.tag == tag && block.status != CacheBlockStatus::Invalid);

        if let Some((index, block)) = cache_block {
            if operation == CacheOp::Write {
                block.status = CacheBlockStatus::Dirty;
            }
            let ready_time = block.ready_time.saturating_sub(now);

            if index != 0 && self.block_replacement == BlockReplacement::Lru {
                // Update LRU order
                let block = cache_set.blocks.remove(index).unwrap();
                cache_set.blocks.push_front(block);

                // Self::update_lru_order(cache_set, index);
            }
            (true, std::cmp::max(self.hit_latency, ready_time), None, 0)
        } else {
            // Cache miss
            let mut lat = 0;
            let mut evicted_block_addr: Option<u64> = None;

            debug_assert!(cache_set.blocks.len() as u64 == self.associativity);

            // Cache set is full
            let random_index = self.rng.gen_range(0..self.associativity) as usize;
            let evicted_block = match self.block_replacement {
                BlockReplacement::Lru | BlockReplacement::Fifo => {
                    // Update LRU order
                    cache_set.blocks.pop_back().unwrap()
                }
                BlockReplacement::Random => {
                    // Swap the random block with the new block
                    cache_set.blocks.remove(random_index).unwrap()
                }
            };

            if evicted_block.status != CacheBlockStatus::Invalid {
                // Evicted block is valid
                lat = evicted_block.ready_time.saturating_sub(now);

                if evicted_block.status == CacheBlockStatus::Dirty {
                    evicted_block_addr = Some(
                        (evicted_block.tag << self.tag_shift)
                            | (set << self.set_shift)
                            | (0 << self.block_mask),
                    );
                }
            }
            (false, lat, evicted_block_addr, random_index)
        }
    }

    pub fn insert_replacement(
        &mut self,
        address: u64,
        now: Tick,
        op: CacheOp,
        random_index: usize,
    ) {
        let tag = self.cache_tag(address);
        let set = self.cache_set(address);

        let cache_set = &mut self.sets[set as usize];

        assert!(cache_set.blocks.len() as u64 == self.associativity - 1);

        let new_block = CacheBlock {
            tag,
            status: match op {
                CacheOp::Read => CacheBlockStatus::Valid,
                CacheOp::Write => CacheBlockStatus::Dirty,
            },
            ready_time: now,
        };

        match self.block_replacement {
            BlockReplacement::Lru | BlockReplacement::Fifo => {
                // Update LRU order
                cache_set.blocks.push_front(new_block);
            }
            BlockReplacement::Random => {
                // Swap the random block with the new block
                cache_set.blocks.insert(random_index, new_block);
            }
        }
    }
}
