use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        error(err);
        process::exit(1);
    });

    let runner = run(config).unwrap_or_else(|err| {
        error(err);
        process::exit(1);
    });
}

struct Config {
    query: String,
    file_path: String,
}

impl Config { 
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok( Config { query, file_path } )
    }
}

fn run(config: Config) -> Result<bool, &'static str> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file but could not");

    println!("{contents}");
    Ok(true)
}

fn error(message: &str) {
    println!("Problem parsing arguments: {message}");
}