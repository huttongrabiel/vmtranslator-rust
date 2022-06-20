use std::{env, fs};

mod codegen;
mod lexer;
mod parser;

pub struct Config {
    path: String,
    is_file: bool,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Self, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide .vm file or directory path"),
        };

        let is_file = match fs::metadata(&path) {
            Ok(metadata) => metadata.is_file(),
            Err(_) => return Err("Provided .vm file not found, check path"),
        };

        Ok(Self {
            path,
            is_file,
        })
    }

    pub fn is_file(&self) -> bool { self.is_file }
}

pub fn translate(config: Config) -> Result<String, &'static str> {
    let mut translated_contents = String::new();

    read_file(&config.path)
        .lines();

    Ok(translated_contents)
}

fn read_file(path: &String) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => panic!("File not found! {}", e),
    }
}
