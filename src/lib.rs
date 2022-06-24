use std::{env, fs, error::Error};

mod codegen;
mod lexer;
mod parser;

pub struct Config {
    path: String,
    is_file: bool,
    pub files: Vec<String>,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Self, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide .vm file or directory path"),
        };

        let file_metadata = FileMetadata::new(&path);
        let files = file_metadata.files;

        Ok(Self {
            path,
            is_file: true,
            files,
        })
    }

    pub fn is_file(&self) -> bool { self.is_file }
}

pub fn translate(config: &Config) -> Result<String, &'static str> {
    let mut translated_contents = String::new();

    let config_path = &config.path;
    let file_metadata = FileMetadata::new(&config_path);
    let lines = file_metadata.read_file().expect("File dead");

    for line in lines.lines() {
        let tokens = lexer::lex(&line);
        let parsed_line = match parser::ParsedLine::parse(tokens) {
            Ok(pl) => pl,
            Err(e) => {
                return Err(e);
            },
        };
        let assembly_output = match codegen::codegen(&config, &parsed_line) {
            Ok(asm) => asm,
            Err(e) => return Err(e),
        };

        translated_contents.push_str(&assembly_output);
    }

    Ok(translated_contents)
}

struct FileMetadata {
    path: String,
    is_file: bool,
    files: Vec<String>,
}

impl FileMetadata {
    pub fn new(path: &String) -> Self {
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => panic!("Provided .vm file not found, check path"),
        };

        // Just assume directory if not a file
        let is_file = metadata.is_file();

        let files: Vec<String> = if is_file {
            vec![path.to_string()]
        } else {
            match fs::read_dir(&path) {
                Ok(mut itr) => {
                    for i in itr {
                        println!("{:?}", i);
                    }
                    todo!();
                    vec!["happy birthday".to_string()]
                },
                Err(_) => panic!("Could not resolve files in directory"),
            }
        };

        dbg!("{}", &files);

        let path = path.to_string();

        Self {
            path,
            is_file,
            files
        }

    }

    pub fn read_file(&self) -> Result<String, &'static str> {

        match fs::read_to_string(&self.path) {
            Ok(contents) => Ok(contents),
            Err(_) => panic!("File not found!"),
        }
    }
}
