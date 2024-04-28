use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{
    eyre::WrapErr,
    Result,
};
use ratatui::prelude::{Stylize, Widget};

use crate::interactive::App;

mod constants;
mod header;
mod load_commands;
mod mach_o;
mod memory_utils;
mod parser;
mod printer;
mod tui;
mod interactive;
mod errors;

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

// todo!(handle errors with color_eyre or other way...)

fn main() -> Result<()> {
    errors::install_hooks()?;

    let args = Args::parse();
    let mach_o = match File::open(&args.file.as_path()) {
        Ok(mut file) => parser::parse(&mut file),
        Err(e) => panic!("Error opening input file for reading! {}", e),
    };

    if args.interactive {
        let mut terminal = tui::init(&mach_o).expect("Error on interactive mode");
        App::default().run(&mut terminal).expect("Error on interactive mode");
        tui::restore().expect("Error on interactive mode")
    }

    if args.header {
        printer::print_header(&mach_o.header);
    }

    if args.load_commands {
        printer::print_load_commands(&mach_o.load_commands);
    }

    Ok(())
}
