use std::{env, error::Error, fs};

pub struct Config {
    // when some variables united by one idea - it is better to compare them into one structure
    // Note: Using primitive values when a complex type would be more appropriate is an
    // anti-pattern known as primitive obsession.
    pub query: String,    // string is owned type
    pub filename: String, // string is owned type
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if case_sensitive == true {
            for arg in args {
                if arg == "-i" || arg == "--ignore-case" {
                    case_sensitive = false;
                }
            }
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // Reads provided file into the string, if function will panic prints the message
    // in .expect therefore we can conclude that .expect method only for programmer's
    // information
    // for more user-friendly output and error handling capabilities
    // it is better to return Result type

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensetive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // We need lifetime annotation here in function signature, because we need bound
    // function parameter with return value: in this case we return string slice which
    // is part of `content` value, therefore it has to live as long as `contents` lives

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // The `query` is now String as .to_lowercase creates new data,
    // rather than referencing existing data

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
    // Since `query` is String we need to add `&` to the .contains method in order
    // to pass string slice as a parameter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensetive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensetive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, contents)
        );
    }
}
