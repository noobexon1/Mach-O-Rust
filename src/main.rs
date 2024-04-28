#[macro_use] extern crate prettytable;

use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

mod constants;
mod header;
mod load_commands;
mod mach_o;
mod memory_utils;
mod parser;
mod printer;


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

fn main() {
    let args = Args::parse();
    let mach_o = match File::open(&args.file.as_path()) {
        Ok(mut file) => parser::parse(&mut file),
        Err(e) => panic!("Error opening input file for reading! {}", e),
    };

    if args.interactive {
        println!("Not yet implemented!");
    }

    if args.header {
        printer::print_header(&mach_o.header);
    }

    if args.load_commands {
        printer::print_load_commands(&mach_o.load_commands);
    }

}
