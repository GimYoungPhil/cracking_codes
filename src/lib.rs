
use std::error;
use std::fs;

pub mod the_reverse_cipher;

pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct Config {
    pub mode: Mode,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mode = match args.next() {
            Some(arg) => match &arg[..] {
                "encoding" => Mode::Encrypt,
                "decoding" => Mode::Decrypt,
                _ => return Err("Didn't match mode string('encoding' or 'decoding')"),
            },
            None => return Err("Didn't get a mode string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn'g get a file path"),
        };

        Ok(Config {
            mode,
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    let results = match config.mode {
        Mode::Encrypt => the_reverse_cipher::encrypt_message(&contents),
        Mode::Decrypt => the_reverse_cipher::decrypt_message(&contents),
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
