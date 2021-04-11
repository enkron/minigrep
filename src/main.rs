use obs::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // using type annotation Vec<String>, coz .collect() method doesn't know
    // which type of collection we want to produce within it

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // Passing a pointer to whole vector of arguments to the parse_config function,
    // therefore all parsing logic isn't presented inside the main function

    if let Err(e) = obs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
