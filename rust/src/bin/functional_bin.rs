use clap::Parser;
#[derive(Parser, Debug)]
#[command(name = "Riscalar Functional Simulator CLI", author = "Josiah Mendes")]
#[command(version = "0.1.0")]
#[command(about = "A Cycle-Approximate, Parametrisable RISC-V Microarchitecture Explorer & Simulator", long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    /// Path to the binary to be simulated
    pub binary_path: String,
}

pub fn main() {
    let args = Args::parse();
    match riscalar::run_functional_simulation(args.binary_path) {
        Ok((_reg, s)) => {
            let iter: Vec<&str> = s.split("---STATS---").collect();
            print!("{}", iter[0]);
            let stats: serde_json::Value = serde_json::from_str(iter[1]).unwrap();
            println!("{}", serde_json::to_string_pretty(&stats).unwrap());
        }
        Err(e) => {
            let iter: Vec<&str> = e.split("---STATS---").collect();
            print!("{}", iter[0]);
            let stats: serde_json::Value = serde_json::from_str(iter[1]).unwrap();
            println!("{}", serde_json::to_string_pretty(&stats).unwrap());
        }
    }
}
