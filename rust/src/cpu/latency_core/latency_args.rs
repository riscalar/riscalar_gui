use super::components::{
    cache::{BlockReplacement, CacheOptions},
    functional_unit_pool::FunctionalUnitGroupCfg,
};
use clap::{Args, Parser, ValueEnum};
use flutter_rust_bridge::frb;

#[derive(Parser, Debug)]
#[command(name = "Riscalar CLI", author = "Josiah Mendes")]
#[command(version = "0.1.0")]
#[command(about = "A Cycle-Approximate, Parametrisable RISC-V Microarchitecture Explorer & Simulator", long_about = None)]
#[command(next_line_help = true)]
#[frb(dart_metadata=("freezed"))]
pub struct LatencyArgs {
    /// Path to the binary to be simulated
    pub binary_path: String,

    /// Reservation Station Queue Size - determines number of instructions that can be inflight at once
    #[arg(long = "rsq:size", value_parser = greater_than_one_power_of_two)]
    pub rsq_size: Option<u16>,

    /// Load Store Queue Size - determines number of loads and stores that can be inflight at once
    #[arg(long = "lsq:size", value_parser = greater_than_one_power_of_two)]
    pub lsq_size: Option<u16>,

    #[command(flatten)]
    pub run_config: RunConfig,

    #[command(flatten)]
    pub fetch_config: FetchConfig,

    #[command(flatten)]
    pub decode_config: DecodeConfig,

    #[command(flatten)]
    pub issue_config: IssueConfig,

    #[command(flatten)]
    pub commit_config: CommitConfig,

    #[command(flatten)]
    pub functional_unit_pool_config: FunctionalUnitPoolConfig,

    #[command(flatten)]
    pub memory_config: MemoryConfig,

    #[command(flatten)]
    pub cache_config: CacheConfig,
}

#[derive(Args, Debug)]
#[frb(dart_metadata=("freezed"))]
pub struct RunConfig {
    /// Simulate the first <FAST_FORWARD> instructions using the functional simulator
    #[arg(long = "run:ff")]
    pub fast_forward: Option<u64>,

    /// Maximum number of instructions to simulate
    #[arg(long = "run:max")]
    pub max_instrs: Option<u64>,

    /// Statistic collection interval (in cycles) <START> <END>
    #[arg(long = "run:stat-interval", num_args = 3)]
    pub stat_interval: Option<Vec<u64>>,

    /// Statistic collection frequency (in cycles)
    #[arg(long = "run:stat-freq")]
    pub stat_freq: Option<u64>,
}

#[derive(Args, Debug)]
#[frb(dart_metadata=("freezed"))]
pub struct FetchConfig {
    /// Instruction Fetch Dispatch Queue Size
    #[arg(long = "fetch:qsize", value_parser = non_neg_power_of_two_u16)]
    pub fetch_queue_size: Option<u16>,

    /// Fetch Speed Relative to Execution Core
    #[arg(long = "fetch:speed", value_parser = clap::value_parser!(u8).range(1..))]
    pub fetch_speed: Option<u8>,

    /// Penalty applied to fetch unit when a mispredicted branch is encountered
    #[arg(long = "fetch:bp", value_parser = clap::value_parser!(u64).range(1..))]
    pub fetch_branch_penalty: Option<u64>,
}

#[derive(Args, Debug)]
#[frb(dart_metadata=("freezed"))]
pub struct DecodeConfig {
    /// Instruction Decode Width - Number of instructions decoded per cycle
    #[arg(long = "decode:width", value_parser = non_neg_power_of_two_u8)]
    pub decode_width: Option<u8>,
}

#[derive(Args, Debug)]
#[frb(dart_metadata=("freezed"))]
pub struct IssueConfig {
    /// Instruction Issue Width - Number of instructions issued per cycle
    #[arg(long = "issue:width", value_parser = non_neg_power_of_two_u8)]
    pub issue_width: Option<u8>,

    /// Issue Order
    #[arg(long = "issue:order", value_enum, default_value = "outorder")]
    pub issue_order: IssueOrder,

    /// Prevent instructions from being issued down a mispredicted speculative branch
    #[arg(long = "issue:no-misspec")]
    pub issue_no_misspec: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum IssueOrder {
    /// Earlier instructions must have been issued before later instructions
    InOrder,
    /// Allow instructions that have satisfied dependencies to be issued before earlier instructions
    OutOrder,
}

#[derive(Args, Debug)]
#[frb(dart_metadata=("freezed"))]
pub struct CommitConfig {
    /// Instruction Commit Width - Number of instructions committed per cycle
    #[arg(long = "commit:width", value_parser = non_neg_power_of_two_u8)]
    pub commit_width: Option<u8>,
}

#[derive(Args, Debug, Clone)]
#[frb(dart_metadata=("freezed"))]
pub struct FunctionalUnitPoolConfig {
    /// Integer ALU Configuration - sets the issue latency, operation latency, and number of units
    #[arg(long = "fu:ialu",  value_parser=fu_value_parser, default_value = "1:1:4")]
    pub ialu: FunctionalUnitGroupCfg,

    /// Integer Multiplier Configuration - sets the issue latency, operation latency, and number of units
    #[arg(long = "fu:imult", value_parser=fu_value_parser, default_value = "1:3:1")]
    pub imult: FunctionalUnitGroupCfg,

    /// Integer Divider Configuration - sets the issue latency, operation latency, and number of units
    #[arg(long = "fu:idiv", value_parser=fu_value_parser, default_value = "19:20:1")]
    pub idiv: FunctionalUnitGroupCfg,

    /// Memory Load Port Configuration - sets the issue latency, operation latency, and number of units
    #[arg(long = "fu:load",  value_parser=fu_value_parser, default_value = "1:20:2")]
    pub load: FunctionalUnitGroupCfg,

    /// Memory Store Port Configuration - sets the issue latency, operation latency, and number of units
    #[arg(long = "fu:store",  value_parser=fu_value_parser, default_value = "1:20:2")]
    pub store: FunctionalUnitGroupCfg,
}

#[derive(Args, Debug, Clone)]
#[frb(dart_metadata=("freezed"))]
pub struct MemoryConfig {
    /// Memory Bus Width (bytes)
    #[arg(long = "mem:buswidth", default_value = "8")]
    pub memory_bus_width: u8,

    /// Latency of memory accesses in cycles - first chunk takes FIRST_CHUNK_LAT cycles, subsequent chunks take INTER_CHUNK_LAT cycles
    #[arg(long = "mem:lat", num_args = 2, value_names = &["FIRST_CHUNK_LAT", "INTER_CHUNK_LAT"])]
    pub memory_latency: Option<Vec<u64>>,
}

#[derive(Args, Debug, Clone)]
#[frb(dart_metadata=("freezed"))]
pub struct CacheConfig {
    /// L1 I-Cache Configuration - either "unified" or <NUMBER_OF_SETS>:<BLOCK_SIZE>:<ASSOCIATIVITY>:<REPLACEMENT_POLICY>:<LATENCY>
    ///
    /// <NUMBER_OF_SETS> must be a power of two and non-zero
    /// <ASSOCIATIVITY> must be a power of two and non-zero
    /// <BLOCK_SIZE> (bytes) must be a power of two and greater than 8
    /// <REPLACEMENT_POLICY> must be one of "l" (Least Recently Used) or "r" (Random) or "f" (FIFO)
    /// <LATENCY> (cycles) must be greater than 0
    #[arg(long = "cache:l1i", value_parser = cache_opt_parser, default_value="512:32:1:l:1")]
    pub il1: CacheOptions,

    /// L1 D-Cache Configuration - either "unified" or <NUMBER_OF_SETS>:<BLOCK_SIZE>:<ASSOCIATIVITY>:<REPLACEMENT_POLICY>:<LATENCY>
    ///
    /// <NUMBER_OF_SETS> must be a power of two and non-zero
    /// <ASSOCIATIVITY> must be a power of two and non-zero
    /// <BLOCK_SIZE> (bytes) must be a power of two and greater than 8
    /// <REPLACEMENT_POLICY> must be one of "l" (Least Recently Used) or "r" (Random) or "f" (FIFO)
    /// <LATENCY> (cycles) must be greater than 0
    #[arg(long = "cache:l1d", value_parser = cache_opt_parser, default_value="128:32:4:l:1")]
    pub dl1: CacheOptions,

    /// L2 I-Cache Configuration - either "unified" or <NUMBER_OF_SETS>:<BLOCK_SIZE>:<ASSOCIATIVITY>:<REPLACEMENT_POLICY>:<LATENCY>
    ///
    /// <NUMBER_OF_SETS> must be a power of two and non-zero
    /// <ASSOCIATIVITY> must be a power of two and non-zero
    /// <BLOCK_SIZE> (bytes) must be a power of two and greater than 8
    /// <REPLACEMENT_POLICY> must be one of "l" (Least Recently Used) or "r" (Random) or "f" (FIFO)
    /// <LATENCY> (cycles) must be greater than 0
    #[arg(long = "cache:l2i", value_parser = cache_opt_parser, default_value="unified")]
    pub il2: CacheOptions,

    /// L2 D-Cache Configuration - either "unified" or <NUMBER_OF_SETS>:<BLOCK_SIZE>:<ASSOCIATIVITY>:<REPLACEMENT_POLICY>:<LATENCY>
    ///
    /// <NUMBER_OF_SETS> must be a power of two and non-zero
    /// <ASSOCIATIVITY> must be a power of two and non-zero
    /// <BLOCK_SIZE> (bytes) must be a power of two and greater than 8
    /// <REPLACEMENT_POLICY> must be one of "l" (Least Recently Used) or "r" (Random) or "f" (FIFO)
    /// <LATENCY> (cycles) must be greater than 0
    #[arg(long = "cache:l2d", value_parser = cache_opt_parser, default_value="1024:64:4:l:6")]
    pub dl2: CacheOptions,
}

fn non_neg_power_of_two_u8(s: &str) -> Result<u8, String> {
    let n = s
        .parse::<u8>()
        .map_err(|_| format!("Provided value '{s}' is not a number"))?;
    if n.is_power_of_two() {
        Ok(n)
    } else {
        Err(format!("Provided value '{s}' is not a power of two",))
    }
}

fn non_neg_power_of_two_u16(s: &str) -> Result<u16, String> {
    let n = s
        .parse::<u16>()
        .map_err(|_| format!("Provided value '{s}' is not a number"))?;
    if n.is_power_of_two() {
        Ok(n)
    } else {
        Err(format!("Provided value '{s}' is not a power of two",))
    }
}

fn greater_than_one_power_of_two(s: &str) -> Result<u16, String> {
    let n = s
        .parse::<u16>()
        .map_err(|_| format!("Provided value '{s}' is not a number"))?;
    if n.is_power_of_two() && n > 1 {
        Ok(n)
    } else {
        Err(format!(
            "Provided value '{s}' is not a power of two or less than 1",
        ))
    }
}

fn fu_value_parser(s: &str) -> Result<FunctionalUnitGroupCfg, String> {
    let split: Vec<&str> = s.split(':').collect();
    if split.len() != 3 {
        return Err("Functional Unit Group configuration must have 3 values - <ISSUE_LAT>:<OP_LAT>:<UNIT_COUNT>".to_string());
    }
    let issue_latency = split[0].parse::<u64>().map_err(|_| {
        format!("Provided value '{s}' for functional unit issue latency is not valid")
    })?;
    let operation_latency = split[1].parse::<u64>().map_err(|_| {
        format!("Provided value '{s}' for functional unit operation latency is not valid")
    })?;
    let num_units = split[2]
        .parse::<usize>()
        .map_err(|_| format!("Provided value '{s}' for number of functional units is not valid"))?;

    Ok(FunctionalUnitGroupCfg {
        issue_latency,
        operation_latency,
        num_units,
    })
}

fn cache_opt_parser(s: &str) -> Result<CacheOptions, String> {
    match s {
        "unified" => Ok(CacheOptions::Unified),
        "none" => Ok(CacheOptions::None),
        _ => {
            let split: Vec<&str> = s.split(':').collect();

            if split.len() != 5 {
                return Err(
                    "Cache configuration must either have 5 values or 'unified' or 'none' "
                        .to_string(),
                );
            }

            let num_sets = split[0]
                .parse::<u64>()
                .map_err(|_| format!("Provided value '{s}' for number of sets is not a number"))?;
            let block = split[1]
                .parse::<u64>()
                .map_err(|_| format!("Provided value '{s}' for block size is not a number"))?;
            if block < 8 {
                return Err("Cache block size must be at least 8 bytes".to_string());
            }
            let assoc = split[2]
                .parse::<u64>()
                .map_err(|_| format!("Provided value '{s}' for associativity is not a number"))?;
            let repl = match split[3] {
                "l" => BlockReplacement::Lru,
                "r" => BlockReplacement::Random,
                "f" => BlockReplacement::Fifo,
                _ => return Err("Cache replacement policy must be one of 'l'(Least Recently Used), 'r'(Random), or 'f'(FIFO)".to_string()),
            };
            let lat = split[4]
                .parse::<u64>()
                .map_err(|_| format!("Provided value '{s}' for latency is not a number"))?;
            if num_sets.is_power_of_two() && assoc.is_power_of_two() && block.is_power_of_two() {
                Ok(CacheOptions::Configured {
                    num_sets,
                    associativity: assoc,
                    block_size: block,
                    replacement_policy: repl,
                    latency: lat,
                })
            } else {
                Err("Cache size, associativity, and block size must be powers of two".to_string())
            }
        }
    }
}
