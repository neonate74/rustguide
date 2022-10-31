use std::{fs::File, io::prelude::*, process, error::{Error}};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn _new (args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let temp = std::env::var("CASE_INSENSITIVE");
        println!("{:?}", temp);

        let mut case_sensitive = temp.is_err();
        if let Ok(_str) = temp {
            if _str == "1" {
                case_sensitive = false;
            }
            else {
                case_sensitive = true;
            }
        }
    
        Ok(Config { query, filename, case_sensitive })
    }

    pub fn new(args: &[String]) -> Config {
        Self::_new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        })
    }
}

pub fn run(config: Config) {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = &self::_run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn _run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file_contents(&config.filename)?;

    if config.case_sensitive {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    }
    else {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn read_file_contents(cfg: &str) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(&cfg)?;
    // .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    // .expect("something went wrong reading the file");

    // println!("[With text:]\n{}", contents);
    Ok(contents)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test {
    // use std::fmt::Error;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let result = read_file_contents("poem.txt");
        if let Ok(_str) = result {
            assert_eq!(vec!["How dreary to be somebody!"], search("Dreary", &_str));
        }
        else if let Err(err) = result {
            eprintln!("{:?}", err);
            assert!(false, "The error was occurred: {:?}", err);
        }
    }

    #[test]
    fn case_insensitive() {
        let result = read_file_contents("poem.txt");
        if let Ok(_str) = result {
            assert_eq!(vec!["How dreary to be somebody!"], search_case_insensitive("Dreary", &_str));
        }
        else if let Err(err) = result {
            eprintln!("{:?}", err);
            assert!(false, "The error was occurred: {:?}", err);
        }
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String
    }

    fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|x| x.size == shoe_size)
            .collect()
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {size: 10, style: "sneaker".to_string()},
            Shoe {size: 13, style: "sandal".to_string()},
            Shoe {size: 10, style: "boot".to_string()},
        ];

        let in_my_size = shoe_in_my_size(shoes, 10);

        assert_eq!(in_my_size,
            vec![
                Shoe {size: 10, style: "sneaker".to_string()},
                Shoe {size: 10, style: "boot".to_string()}
            ]
        );
    }
}
