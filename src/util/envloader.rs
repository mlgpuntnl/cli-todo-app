use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

pub fn load_env() {
    let env_file = load_env_file();
    let reader = BufReader::new(env_file);
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => handle_line(line),
            Err(error) => eprintln!("Failed to read line: {:?}", error),
        }
    }
}

fn load_env_file() -> File {
    let env_file_result = File::open(".env");
    match env_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Error: .env file not found"),
            other_error => panic!("Error: failed to open .env file: {:?}", other_error),
        },
    }
}

fn handle_line(line: String) {
    let line_parts: Vec<&str> = line.split('=').collect();
    if line_parts.len() != 2 {
        panic!(
            "Error: .env line needs to consist of two parts. Got: {:?}",
            line_parts
        );
    }
    env::set_var(
        line_parts.first().unwrap().to_uppercase(),
        line_parts.get(1).unwrap(),
    );
}
