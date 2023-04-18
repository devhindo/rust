#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = CLI::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Oh noes: {}", error); }
    };
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
