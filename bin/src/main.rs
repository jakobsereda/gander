use clap::Parser;
use std::fs;

/// A compiler for the Gander programming language
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to .gand source file
    #[arg(short, long)]
    path: String,

    /// Turns debugging info on
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let data = fs::read_to_string(args.path).expect("Unable to read from provided filepath");

    println!("{}", data);
}
