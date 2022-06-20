use std::error::Error;
use std::{env, process};

use vm_translator::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = match Config::new(&mut env::args()) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Problem parsing CLI arguments: {}", e);
            process::exit(1);
        },
    };

    let translated_content = match vm_translator::translate(config) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Error occured during translation: {}", e);
            process::exit(1)
        },
    };

    Ok(())
}
