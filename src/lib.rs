use std::path::PathBuf;

use clap::Parser;

/// A command-line tool written in rust to view and explore files with mach-o format.
#[derive(Parser)]
#[command(name = "Mach_O_Rust")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input mach-o file
    #[arg(short, long, value_name = "FILE_PATH")]
    pub input: PathBuf,
}

pub fn run(args: &Args) {}
