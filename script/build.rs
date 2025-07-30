use std::env;
use sp1_build::build_program_with_args;

fn main() {
    // Skip building guest programs if proving remotely
    let prover = env::var("SP1_PROVER").unwrap_or_default();
    if prover == "network" {
        println!("cargo:warning=Skipping guest program build since SP1_PROVER=network");
        return;
    }

    // Only build when proving locally
    build_program_with_args("../program", Default::default());
    build_program_with_args("../Icr", Default::default());
    build_program_with_args("../Liquid", Default::default());
    build_program_with_args("../Real_time_ltv", Default::default());
    build_program_with_args("../BimaOccumPutCall", Default::default());
}
