use std::{process, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // parsing input
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem passing arguments {}",err);
        process::exit(1);
    });

    run(config);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
}