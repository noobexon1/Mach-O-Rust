use clap::Parser;

use mach_o_rust::{Args, run};

fn main() {
    let args = Args::parse();
    run(&args);
}