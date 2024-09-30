use std::fs;

#[derive(Debug)]
pub enum GrepError {
    ConfigError(&'static str),
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut arg_iter: impl Iterator<Item = String>) -> Result<Config, GrepError> {
        if arg_iter.size_hint().0 < 3 {
            return Err(GrepError::ConfigError("Not enough arguments"));
        }
        arg_iter.next(); // skip the program name

        let query = arg_iter.next().unwrap();
        let filename = arg_iter.next().unwrap();
        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), GrepError> {
    let content = fs::read_to_string(config.filename)
        .map_err(|_| GrepError::ConfigError("File not found"))?;
    let results = search(&config.query, &content);
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec![
            "minigrep".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
