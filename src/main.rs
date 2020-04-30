use std::{process, env};

use minigrep::{Config,run};

fn main() {
    let args: Vec<String> = env::args().collect();
    // parsing input
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err); 
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Problem parsing arguments: {}", e); 
        process::exit(1);
    }
    
}


// let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
// can set env vars and import in our program

// eprintln!("Problem parsing arguments: {}", err); 
// can write 