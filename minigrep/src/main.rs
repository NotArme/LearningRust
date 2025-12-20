use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(());
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); //skip over args[0] (name of the executable)
        
        let query = match args.next() {
            Some(a) => a,
            None => return Err("No query string received")
        };

        let file_path = match args.next() {
            Some(a) => a,
            None => return Err("No file_path string received")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok( Config { query, file_path, ignore_case });
    }
}