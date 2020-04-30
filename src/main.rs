use std::{process, env};

use minigrep::{Config,run};

fn main() {
    let args: Vec<String> = env::args().collect();
    // parsing input
    let config = Config::new(&args).unwrap_or_else(|_err|{
        process::exit(1);
    });

    if let Err(_e) = run(config) {
        process::exit(1);
    }
    
}