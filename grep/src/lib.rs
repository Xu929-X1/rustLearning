use std::error::Error;
use std::{fs, env};
pub struct Config {
    pub path: String,
    pub pattern: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let path = &args[1].clone();
        let pattern = &args[2].clone();
        let is_case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        Ok(Config {
            path: path.to_string(),
            pattern: pattern.to_string(),
            is_case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;
    let res: Vec<&str> = if config.is_case_sensitive {
        search(&config.pattern, &content)
    } else {
        case_insensitive_search(&config.pattern, &content)
    };

    for line in res {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line.trim());
        }
    }

    res
}

pub fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line.trim());
        }
    }

    res
}

//Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ductive";
        let content = "\n
        Rust:\n
        safe, fast, productive\n
        Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, content))
    }

    #[test]
    fn case_sensitive() {
        let query = "and";
        let content = "
        When, in disgrace with fortune and men’s eyes,
        I all alone beweep my outcast state,
        And trouble deaf heaven with my bootless cries";

        assert_eq!(
            vec!["When, in disgrace with fortune and men’s eyes,"],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "and";
        let content = "When, in disgrace with fortune and men’s eyes,
        I all alone beweep my outcast state,
        And trouble deaf heaven with my bootless cries";

        assert_eq!(
            vec![
                "When, in disgrace with fortune and men’s eyes,",
                "And trouble deaf heaven with my bootless cries"
            ],
            case_insensitive_search(query, content)
        )
    }
}
