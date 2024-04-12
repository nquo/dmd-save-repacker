use std::{env, process};

use dmdsave::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = dmdsave::run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}
