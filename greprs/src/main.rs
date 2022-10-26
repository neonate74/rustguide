use std::{env, fs::File, io::prelude::*, process, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::create(&args);

    run(config);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }

    fn create(args: &[String]) -> Config {
        Self::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        })
    }
}

fn run(config: Config) {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = &self::_run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn _run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    // .expect("file not found");

    let mut contents = String::new();
    
    f.read_to_string(&mut contents)?;
    // .expect("something went wrong reading the file");

    println!("[With text:]\n{}", contents);

    Ok(())
}