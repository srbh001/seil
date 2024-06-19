mod lexer;
mod parser;

use crate::lexer::{Lexer, Processor, Token, TokenStream};
use crate::parser::Parser;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let file_name = "./src/test/test.asm";
    let mut file = File::open(file_name)?;

    let mut sample = String::new();
    file.read_to_string(&mut sample)?;
    println!("File content:\n{}", sample);

    let mut token_stream = TokenStream::new();
    let mut lexer = Lexer::new(&sample.as_str());

    let line_iter = sample.lines();

    let mut parser = Parser::new(&sample.as_str());

    match parser.parse() {
        Ok(_) => println!("Parsed successfully"),
        Err(e) => {
            let line_number = e.line_number;
            let column_number = e.column_number;

            if let Some(line) = line_iter.clone().nth(line_number - 1) {
                let column_count = count_char_columns(line, column_number);
                println!("--> at {}:{}:{}", file_name, line_number, column_count);
                println!("{}", line);
                println!("{: <1$}^", "", column_count);
                println!("[Parsing Error]: {:?}\n", e.message);
            } else {
                println!("Error: Could not fetch the line for the error.");
            }
        }
    }

    println!(
        "\nInstructions: {:?}\nLabels: {:?}\nTokenStream: {:?}",
        parser.instructions, parser.labels, parser.token_stream
    );

    Ok(())
}

fn count_char_columns(line: &str, column_count: usize) -> usize {
    let mut count = 0;
    let mut remaining_columns = column_count - 1;

    // Skip initial spaces
    while line.chars().nth(count) == Some(' ') {
        count += 1;
    }
    if count > 0 && remaining_columns > 0 {
        remaining_columns -= 1;
    }

    while let Some(c) = line.chars().nth(count) {
        if remaining_columns == 0 {
            break;
        }
        count += 1;
        if c == ':' || c == ',' {
            remaining_columns -= 1;
            count += 1;
            while line.chars().nth(count) == Some(' ') {
                count += 1;
            }
        }
        if c == ' ' {
            remaining_columns -= 1;
            // Skip the spaces
            while line.chars().nth(count) == Some(' ') || line.chars().nth(count) == Some(',') {
                count += 1;
            }
        }
    }
    count
}
