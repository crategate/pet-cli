use std::{env, process};

use pet_cli::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for : {}", config.query);
    println!("in dis file : {}", config.filename);
    if let Err(e) = pet_cli::run(config) {
        println!("App Error: {}", e);
        process::exit(1);
    }
}
