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
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        // Using the .clone() method to copy borrowed value from &[String]
        // It is not efficient at all but for simplicity it is ok

        let filename = args[2].clone();

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if case_sensitive == true {
            for arg in args.iter() {
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
    // Reads provided file into the string, if function will panic prints the message in .expect
    // therefore we can conclude that .expect method only for programmer's information
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
    // We need lifetime annotation here in function signature, because we need bound function
    // parameter with return value: in this case we return string slice which is part of
    // `content` value, therefore it has to live as long as `contents` lives
    let mut results = Vec::new();

    for line in contents.lines() {
        // Iterate through each line of the contents.
        if line.contains(query) {
            // Check whether the line contains our query string.
            results.push(line);
            // If it does, add it to the list of values we’re returning.
            // If it doesn’t, do nothing.
        }
    }

    results // Return the list of results that match.
}

pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // The `query` is now String as .to_lowercase creates new data,
    // rather than referencing existing data

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            // Since `query` is String we need to add `&` to the .contains method in order
            // to pass string slice as a parameter
            results.push(line);
        }
    }

    results
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

    #[test]
    fn return_true_struct_fields() {
        let params = [
            String::from("zero"),
            String::from("one"),
            String::from("two"),
        ];
        let config = Config::new(&params).unwrap();

        assert_eq!(
            vec![String::from("one"), String::from("two")],
            vec![config.query, config.filename]
        );
    }

    #[test]
    #[should_panic]
    fn run_will_panic_without_args() {
        let params = [
            String::from("zero"),
            String::from("one"),
            String::from("two"),
        ];

        let config = Config::new(&params).unwrap();
        run(config).unwrap();
    }
}
