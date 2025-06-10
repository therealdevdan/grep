use std::env;
use std::process;

use minigrep::Config;

fn main() {    
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &'static str| {
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
