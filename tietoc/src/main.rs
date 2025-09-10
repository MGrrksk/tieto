mod token;
mod lexer;
mod value;
mod opcodes;
mod bytecode;

use std::fs;
use crate::{lexer::Lexer, token::TokenType};

fn main() {
    // read file's content
    let content_lines: Vec<String> = match fs::read_to_string("../test.tiet") {
        Ok(string) => string.lines().map(str::to_owned).collect(),
        Err(_) => panic!("Problem while reading a file.")
    };
    // tokenize code
    let mut lexer = Lexer::new(&content_lines.join("\n"));
    loop {
        let token = lexer.get_token();
        println!("{token:?}");
        if token.ttype == TokenType::EOF {break}
    }
}