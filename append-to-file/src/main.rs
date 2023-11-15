use std::env;
use append_to_file::{Config, append_line};

fn main() {
    let args: Vec<String> = env::args().collect();
    match Config::from(&args) {
        Ok(config) => run(&config),
        Err(msg) => eprintln!("{}", msg),
    }
    println!("Hello, world!");
}

fn run(config: &Config) {
    if let Err(msg) = append_line(&config.file, &config.line) {
        eprintln!("{}", msg);
    }
}
