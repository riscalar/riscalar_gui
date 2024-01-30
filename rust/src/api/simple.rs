use crate::{
    cpu::{latency_core::latency_args::LatencyArgs, register::XRegisterFile},
    run_functional_simulation, run_latency_simulation,
};

pub enum SimulationResult {
    Success(XRegisterFile, String, String),
    Error(String, String),
}

pub fn run_simulation_functional(binary_path: String) -> SimulationResult {
    match run_functional_simulation(binary_path) {
        Ok((reg, s)) => {
            let iter: Vec<&str> = s.split("---STATS---").collect();
            SimulationResult::Success(reg, iter[0].to_string(), iter[1].to_string())
        }
        Err(e) => {
            let iter: Vec<&str> = e.split("---STATS---").collect();
            SimulationResult::Error(iter[0].to_string(), iter[1].to_string())
        }
    }
}

pub fn run_simulation_latency(args: LatencyArgs) -> SimulationResult {
    match run_latency_simulation(args) {
        Ok((reg, s)) => SimulationResult::Success(reg, s, "".to_string()),
        Err(e) => SimulationResult::Error(e, "".to_string()),
    }
}
