/*
grep: globally search a regular expression and print
takes 2 arguments:
1. filename
2. string pattern
*/

use std::env;
use std::process;
use grep::{Config, run};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config).unwrap_or_else(|err|{
        println!("Problem when reading file: {:?}", err.to_string());
        process::exit(1);
    });
}

