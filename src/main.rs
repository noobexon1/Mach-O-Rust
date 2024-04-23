use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

mod parser;
mod header;
mod load_commands;
mod constants;
mod printer;

/// A command-line tool written in rust to view and explore files with mach-o format.
#[derive(Parser)]
#[command(name = "Mach_O_Rust")]
#[command(version, about, long_about = None)]
struct Args {
    /// Input mach-o file
    #[arg(short, long, value_name = "FILE_PATH", required = true)]
    input: PathBuf,
    /// Print the mach-o header
    #[arg(short = 'f', long = "header", required = false)]
    header: bool,
}

fn main() {
    let args = Args::parse();
    let mach_o = match File::open(&args.input.as_path()) {
        Ok(mut file) => parser::parse(&mut file),
        Err(e) => panic!("Error opening input file for reading!"),
    };

    if args.header {
        printer::print_header(mach_o.get_header());
    }
}