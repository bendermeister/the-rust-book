use std::error::Error;
use std::fs;
use std::env;

pub struct Config<'a> {
    path: &'a str,
    query: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, String> {
        if args.len() != 3 {
            return Err(String::from("Not enough arguemnts"));
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            path: &args[2],
            query: &args[1],
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = match fs::read_to_string(&config.path) {
        Err(err) => return Err(Box::new(err)),
        Ok(content) => content,
    };

    let v = if config.ignore_case {
        search_case_insensitive(config.query, &content)
    } else {
        search(config.query, &content)
    };

    for line in v {
        println!("{line}")
    }


    Ok(())
}

pub fn search<'a>(query: &str, haystack: &'a str) -> Vec<&'a str> {
    haystack.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, haystack: &'a str) -> Vec<&'a str> {
    haystack
        .lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
