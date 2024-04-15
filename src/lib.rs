use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use crate::parser::parse;

mod parser;
mod translate;

/// A command-line tool written in rust to view and explore files with mach-o format.
#[derive(Parser)]
#[command(name = "Mach_O_Rust")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input mach-o file
    #[arg(short, long, value_name = "FILE_PATH", required = true)]
    pub input: PathBuf,
}

pub fn run(args: &Args) {
    match File::open(&args.input.as_path()) {
        Ok(mut file) => {
            parse(&mut file);
        },
        Err(e) => {
            eprintln!("Failed to open input file: {}", e);
        }
    }
}

