use std::error::Error;
use std::fs;
use std::env;

pub struct Config<'a> {
    pub search_string: &'a str, // заимствует строку. Данные, на которые ссылается Config, должны 
    pub file_path: &'a str,     // жить не меньше, чем сам Config (это гарантирует время жизни 'a).
    pub ignore_case: bool,
}
// # ОСТОРОЖНО!
//
// Если args уничтожатся (например, выйдут из области видимости), а Config останется 
// — программа попытается прочитать уже несуществующие данные (undefined behavior, 
// Rust этого не допустит на этапе компиляции).

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            search_string: &args[1], 
            file_path: &args[2],
            ignore_case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.search_string, &contents)
    } else {
        search(&config.search_string, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.contains(search_string) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    search_string: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let search_string = search_string.to_lowercase();
    let mut resuls: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search_string) {
            resuls.push(line); 
        }
    }
    resuls
}
