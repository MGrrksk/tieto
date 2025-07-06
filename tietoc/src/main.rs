mod token;
mod lexer;

use std::fs;
use crate::{lexer::Lexer, token::Token};

fn main() {
    let content = match fs::read_to_string("../test.tiet") {
        Ok(string) => string,
        Err(_) => panic!("Problem while reading a file.")
    };
    println!("{content}");
    let mut lexer = Lexer::new(&content);
    loop {
        let token = lexer.get_token();
        println!("{token:?}");
        if let Token::EOF(_) = token {break}
    }
}