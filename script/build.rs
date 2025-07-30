use sp1_build::build_program_with_args;

fn main() {
    std::env::set_var("RUSTFLAGS", "-C linker=rust-lld");
    build_program_with_args("../program", Default::default());
    build_program_with_args("../Icr", Default::default());
    build_program_with_args("../Liquid", Default::default());
    build_program_with_args("../Real_time_ltv", Default::default());
    build_program_with_args("../BimaOccumPutCall", Default::default());
}
