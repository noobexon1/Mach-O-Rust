use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use crate::error::AppError;
use crate::mach_o::MachO;

mod constants;
mod header;
mod load_commands;
mod mach_o;
mod memory_utils;
mod parser;
mod printer;
mod error;

/// A command-line tool written in Rust to view and explore mach-o files.
#[derive(Parser)]
#[command(name = "Mach_O_Rust")]
#[command(version, about, long_about = None)]
struct Args {
    /// Input mach-o file
    #[arg(short, long, value_name = "PATH", required = true)]
    file: PathBuf,
    /// Interactive mode
    #[arg(short, long, required = false)]
    interactive: bool,
    /// Print mach-o header
    #[arg(short = 'e', long, required = false)]
    header: bool,
    /// Print mach-o load commands
    #[arg(short, long, required = false)]
    load_commands: bool,
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run(args: &Args) -> Result<(), AppError> {
    let mach_o = parse_from_file(&args.file)?;

    if args.interactive {
        println!("Not yet implemented!");
    }

    if args.header {
        printer::print_header(&mach_o.header);
    }

    if args.load_commands {
        printer::print_load_commands(&mach_o.load_commands);
    }

    Ok(())
}

fn parse_from_file(path: &PathBuf) -> Result<MachO, AppError> {
    let mut file = File::open(path)?;
    let mach_o = parser::parse(&mut file)?;
    Ok(mach_o)
}
