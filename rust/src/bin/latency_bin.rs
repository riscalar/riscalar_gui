use clap::Parser;
use riscalar::{cpu::latency_core::latency_args::LatencyArgs, run_latency_simulation};

pub fn main() {
    let args = LatencyArgs::parse();

    match run_latency_simulation(args) {
        Ok((_reg, s)) => {
            println!("{}", s);
        }
        Err(e) => {
            println!("error!, {}", e)
        }
    }
}
