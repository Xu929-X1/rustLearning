use std::env;
use text_colorizer::Colorize;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "replace".green()
    );
    eprintln!("Usage: replace<target> <replacement> <Input> <Output>");
}
fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expect 4, got {}",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}
