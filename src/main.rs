use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

/// A command-line tool written in rust to view and explore files with mach-o format.
#[derive(Parser)]
#[command(name = "Mach_O_Rust")]
#[command(version, about, long_about = None)]
struct Args {
    /// Input mach-o file
    #[arg(short, long, value_name = "FILE_PATH")]
    input: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE_PATH")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Add verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Adds files to myapp
    Add {
        name: Option<String>
    },
}

fn main() {
    let args = Args::parse();

    if let Some(config_path) = args.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match args.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &args.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Add { name }) => {
            println!("'myapp add' was used, name is: {name:?}")
        }
        None => {}
    }
}

// https://docs.rs/clap/latest/clap/index.html
// TODO: add more options.
// TODO: add more functionality with [subcommands].
// TODO: add interactivity?