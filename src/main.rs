use crate::parser::*;
use crate::scanner::*;
use std::env;
use std::fs;

mod parser;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize/parse <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // TODO: create a Result to see if tokenazation failed somehow ?
            println!("Tokenizer output: {:?}", tokenize(&file_contents));
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tokenizer() {}
}
