use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing error {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config:Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &content) {
        println!("{line}");
    }
    Ok(())
}
struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    fn build(args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        let query = String::from(&args[1]);
        let file_path = String::from(&args[2]);
        Ok(Config{query, file_path})
    }
}

