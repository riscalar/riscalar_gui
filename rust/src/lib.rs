//!
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
use crate::cpu::{functional_core::RunSuccessStatus, latency_core::LatencyCore};
use cpu::{
    exception::Exception, functional_core::FunctionalCore, latency_core::latency_args::LatencyArgs,
    register::XRegisterFile,
};
use serde_json::{json, Value};
use std::{fs::File, io::Read, time::SystemTime};

pub mod api;
pub mod cpu;
mod frb_generated;

pub fn run_functional_simulation(binary_path: String) -> Result<(XRegisterFile, String), String> {
    let (binary, filename) = read_binary_to_vec(binary_path)?;
    let config = json!({"type":"functional"});

    // Mark simulation start time
    let sim_start = SystemTime::now();

    let mut core = FunctionalCore::new();
    core.memory.load_binary(binary);
    core.reset();
    match core.run(None) {
        Ok(_) => Ok((
            core.xreg.clone(),
            sim_success_stringify(&core, sim_start, &filename, config),
        )),
        Err(e) => Err(sim_error_stringify(&core, sim_start, &filename, config, e)),
    }
}

pub fn run_latency_simulation(args: LatencyArgs) -> Result<(XRegisterFile, String), String> {
    let (binary, filename) = read_binary_to_vec(args.binary_path.clone())?;
    let config = json!({"type":"cycle","fast_forward":args.run_config.fast_forward});

    // Mark simulation start time
    let sim_start = SystemTime::now();

    // Initialise latency core as new
    let mut latency_core;

    // Run functional simulation for fast forward
    if args.run_config.fast_forward.is_some() {
        let mut functional_core = FunctionalCore::new();
        functional_core.memory.load_binary(binary);
        functional_core.reset();

        // Begin functional simulation to fast forward
        match functional_core.run(args.run_config.fast_forward) {
            Ok(s) => {
                if let RunSuccessStatus::Complete = s {
                    return Ok((
                        functional_core.xreg.clone(),
                        sim_success_stringify(&functional_core, sim_start, &filename, config),
                    ));
                }
            }
            Err(e) => {
                return Err(sim_error_stringify(
                    &functional_core,
                    sim_start,
                    &filename,
                    config,
                    e,
                ))
            }
        }
        latency_core = LatencyCore::from_functional_core(&functional_core, &args);
    } else {
        latency_core = LatencyCore::new(&args);
        latency_core.memory.load_binary(binary);
        latency_core.reset();
    }

    println!("Starting performance simulation");
    let res = latency_core.run(args);
    println!("{:?}", res);
    match res {
        Ok(s) => match s {
            RunSuccessStatus::Complete => Ok((
                latency_core.xreg.clone(),
                format!(
                    "Success, Cycles: {}, IPC: {},
Executed Instrs:  {},
Committed Instrs: {},
Simulation Time:  {},
Simulation Speed: {},
Simulation Result Registers: {}",
                    latency_core.tick,
                    latency_core.stats.ipc_committed,
                    latency_core.stats.executed_instrs,
                    latency_core.stats.committed_instrs,
                    sim_start.elapsed().unwrap().as_secs_f64(),
                    latency_core.stats.executed_instrs as f64
                        / sim_start.elapsed().unwrap().as_secs_f64(),
                    latency_core.xreg
                ),
            )),
            RunSuccessStatus::EarlyTermination => Ok((
                latency_core.xreg.clone(),
                format!(
                    "Early Termination, Cycles: {}, Simulation Time: {},
                    Simulation Result Registers: {}",
                    latency_core.tick,
                    sim_start.elapsed().unwrap().as_secs_f64(),
                    latency_core.xreg
                ),
            )),
        },
        Err(e) => Err(format!("error {}", e)),
    }
}

fn read_binary_to_vec(binary_path: String) -> Result<(Vec<u8>, String), String> {
    let mut file: File = File::open(binary_path.clone()).map_err(|e| e.to_string())?;
    let filename: &str = binary_path.split('/').last().unwrap();
    let mut binary: Vec<u8> = Vec::new();
    file.read_to_end(&mut binary).map_err(|e| e.to_string())?;
    Ok((binary, filename.to_string()))
}

fn add_sim_info(
    current: Value,
    sim_start: SystemTime,
    sim_binary_filename: &str,
    config: Value,
) -> Value {
    let mut stats = current;
    stats["simulation_configuration"] = config;
    stats["simulation_binary_filename"] = json!(sim_binary_filename.to_string());
    stats["simulation_start_time_ms"] = json!(sim_start
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros());
    stats["simulation_total_time_ms"] = json!(sim_start.elapsed().unwrap().as_micros());
    stats
}

fn sim_success_stringify(
    core: &FunctionalCore,
    sim_start: SystemTime,
    filename: &str,
    config: Value,
) -> String {
    format!(
        "Simulation completed successfully in {} cycles\n---STATS---\n{}",
        core.tick,
        serde_json::to_string_pretty(
            add_sim_info(core.get_sim_res_json(), sim_start, filename, config)
                .as_object()
                .unwrap()
        )
        .unwrap()
    )
}

fn sim_error_stringify(
    core: &FunctionalCore,
    sim_start: SystemTime,
    filename: &str,
    config: Value,
    exception: Exception,
) -> String {
    format!(
        "Simulation terminated on cycle {} @ instr {} with error {}\n---STATS---\n{}",
        core.tick,
        core.pc,
        exception,
        serde_json::to_string_pretty(
            add_sim_info(core.get_sim_res_json(), sim_start, filename, config)
                .as_object()
                .unwrap()
        )
        .unwrap()
    )
}
