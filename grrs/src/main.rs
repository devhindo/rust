#![allow(unused)]
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct CLI {
    pattern: String, /// the pattern to look for
    path: std::path::PathBuf, // the path to the file to read
}
fn main() {
    let args = CLI::parse();

}
