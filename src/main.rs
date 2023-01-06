use std::env;
use std::process;
use std::error::Error;

use swangrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = swangrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn error(message: &str) {
    println!("Problem parsing arguments: {message}");
}