use std::{path::Path, fs};
use clap::{App, Arg};
use rmd::parser::lex;

fn main() {
    let res = App::new("rmd")
        .arg(
            Arg::with_name("file_path")
                .index(1)
                .validator(validate_file_path),
        )
        .try_get_matches();

    // throw err if parsing or validation failed
    if res.is_err() {
        res.unwrap_err().exit();
    }

    let res = res.unwrap();
    let path_as_str = res.value_of("file_path");
    match path_as_str {
        Some(x) => {
            let res = fs::read(x);
            if res.is_ok() {
                lex::parse(&res.unwrap())
            }
            panic!("failed to read file!");
        },
        None => panic!("failed to cast path str to PathBuf!"),
    }
}

fn validate_file_path(v: &str) -> Result<(), String> {
    if Path::new(v).exists() {
        return Ok(());
    }
    Err(String::from(
        "the file must exist!",
    ))
}

#[cfg(test)]
mod validation_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_exists() {
        assert!(validate_file_path("./src/main.rs").is_ok());
    }

    #[test]
    fn test_dne() {
        assert!(validate_file_path("./src/dne.rs").is_err());
    }
}
