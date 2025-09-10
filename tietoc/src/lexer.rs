use crate::token::{Token, TokenType};

pub struct Lexer {
    src: Vec<char>,
    line: u16,
    column: u16,
    start: usize,
    curr: usize
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.chars().collect(),
            line: 1,
            column: 1,
            start: 0,
            curr: 0
        }
    }
    pub fn get_token(&mut self) -> Token {
        self.skip();
        if self.curr >= self.src.len() {return self.token(TokenType::EOF);}
        self.start = self.curr;
        match self.eat() {
            '(' => self.token(TokenType::LPAREN),
            ')' => self.token(TokenType::RPAREN),
            '[' => self.token(TokenType::LBRACK),
            ']' => self.token(TokenType::RBRACK),
            '{' => self.token(TokenType::LBRACE),
            '}' => self.token(TokenType::RBRACE),
            ',' => self.token(TokenType::COMMA),
            '.' => {
                if self.next_is('.') {self.token(TokenType::DBLDOT)}
                else {self.token(TokenType::DOT)}
            },
            ':' => self.token(TokenType::COLON),
            ';' => self.token(TokenType::SEMICOLON),
            '?' => self.token(TokenType::QUESTION),
            '+' => self.token(TokenType::PLUS),
            '-' => self.token(TokenType::MINUS),
            '*' => {
                if self.next_is('*') {self.token(TokenType::DBLSTAR)}
                else {self.token(TokenType::STAR)}
            },
            '/' => {
                if self.next_is('/') {self.token(TokenType::DBLSLASH)}
                else {self.token(TokenType::SLASH)}
            },
            '%' => self.token(TokenType::PRECENT),
            '=' => {
                if self.next_is('=') {self.token(TokenType::DBLEQUAL)}
                else if self.next_is('>') {self.token(TokenType::ARROW)}
                else {self.token(TokenType::EQUAL)}
            },
            '>' => {
                if self.next_is('=') {self.token(TokenType::GTEQUAL)}
                else {self.token(TokenType::GREATER)}
            },
            '<' => {
                if self.next_is('=') {self.token(TokenType::LSEQUAL)}
                else {self.token(TokenType::LESS)}
            },
            '!' => {
                if self.next_is('=') {self.token(TokenType::BNGEQUAL)}
                else {self.token(TokenType::BANG)}
            },
            '|' => {
                if self.next_is('>') {self.token(TokenType::PIPEFORWARD)}
                else {self.token(TokenType::ERROR("Unrecognized character: `|`".to_string()))}
            },
            '"' => {
                while self.curr < self.src.len() && self.src[self.curr] != '"' {
                    if self.src[self.curr] == '\n' {return self.token(TokenType::ERROR("Unterminated string".to_string()))}
                    self.eat();
                }
                if self.curr >= self.src.len() {self.token(TokenType::ERROR("Unterminated string".to_string()))}
                else {
                    self.eat();
                    self.token(TokenType::STRING(String::from_iter(&self.src[self.start+1..self.curr-1])))
                }
            },
            '\n' => {
                let token = self.token(TokenType::NEWLINE);
                self.line += 1;
                self.column = 1;
                token
            },
            '0'..='9' => {
                while self.curr < self.src.len() && self.src[self.curr].is_ascii_digit() {self.eat();}
                if self.curr < self.src.len() - 1 && self.src[self.curr] == '.' && self.src[self.curr+1].is_ascii_digit() {
                    self.eat();
                    while self.curr < self.src.len() && self.src[self.curr].is_ascii_digit() {self.eat();}
                    self.token(TokenType::FLOAT(String::from_iter(&self.src[self.start..self.curr]).parse().unwrap()))
                } else {self.token(TokenType::INT(String::from_iter(&self.src[self.start..self.curr]).parse().unwrap()))}
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                while self.curr < self.src.len() && (self.src[self.curr].is_ascii_alphanumeric() || self.src[self.curr] == '_') {self.eat();}
                let word = String::from_iter(&self.src[self.start..self.curr]);
                match word.as_str() {
                    "u1" => self.token(TokenType::U1),
                    "u8" => self.token(TokenType::U8),
                    "u16" => self.token(TokenType::U16),
                    "u32" => self.token(TokenType::U32),
                    "u64" => self.token(TokenType::U64),
                    "i8" => self.token(TokenType::I8),
                    "i16" => self.token(TokenType::I16),
                    "i32" => self.token(TokenType::I32),
                    "i64" => self.token(TokenType::I64),
                    "f32" => self.token(TokenType::F32),
                    "f64" => self.token(TokenType::F64),
                    "null" => self.token(TokenType::NULL),
                    "and" => self.token(TokenType::AND),
                    "or" => self.token(TokenType::OR),
                    "dyn" => self.token(TokenType::DYN),
                    "if" => self.token(TokenType::IF),
                    "else" => self.token(TokenType::ELSE),
                    "for" => self.token(TokenType::FOR),
                    "while" => self.token(TokenType::WHILE),
                    "var" => self.token(TokenType::VAR),
                    "func" => self.token(TokenType::FUNC),
                    "return" => self.token(TokenType::RETURN),
                    _ => self.token(TokenType::ID(word))
                }
            },
            other => self.token(TokenType::ERROR(format!("Unrecognized character: `{other}`")))
        }
    }
    fn skip(&mut self) {
        while self.curr < self.src.len() {
            match self.src[self.curr] {
                ' ' | '\t' | '\r' => {self.eat();},
                '#' => {while self.curr < self.src.len() && self.src[self.curr] != '\n' {self.eat();}}
                _ => break
            }
        }
    }
    fn eat(&mut self) -> char {
        self.column += 1;
        self.curr += 1;
        self.src[self.curr - 1]
    }
    fn next_is(&mut self, chr: char) -> bool {
        if self.src[self.curr] == chr {
            self.eat();
            true
        } else {false}
    }
    fn token(&self, ttype: TokenType) -> Token {
        Token {ttype, line: self.line, column: self.column - (self.curr - self.start) as u16, length: (self.curr - self.start) as u16}
    }
}