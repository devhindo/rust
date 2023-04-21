use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: my-cli-tool <command>");
        return;
    }

    let command = args[1].clone();

    match Command::new("cmd").args(&[
        "/c",
        &format!("my-cli-tool {}", command),
    ]).output() {
        Ok(output) => {
            if output.status.success() {
                println!("Success!");
            } else {
                println!("Error!");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}