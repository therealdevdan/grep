use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(&args).unwrap_or_else(|err: &'static str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Serching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e)  = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// git config --global user.name "therealdevdan"
// git config --global user.email "alexanderdanilchenko888@gmail.com"