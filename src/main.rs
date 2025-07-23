use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = match Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing args: {err}");
            process::exit(1);
        }
    };
    
    if let Err(e) = minigrep::run(config) { 
        eprintln!("Application error: {e}"); 
        process::exit(1);
    }
}
