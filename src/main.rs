use clap::Parser;

use mach_o_rust::Args;
use mach_o_rust::run;

fn main() {
    let args = Args::parse();
    run(&args);
}