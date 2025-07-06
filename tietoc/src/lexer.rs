use crate::token::{Token, TokenMeta};

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
        if self.curr >= self.src.len() {return Token::EOF(self.tokenmeta());}
        self.start = self.curr;
        match self.eat() {
            '(' => Token::LPAREN(self.tokenmeta()),
            ')' => Token::RPAREN(self.tokenmeta()),
            '[' => Token::LBRACK(self.tokenmeta()),
            ']' => Token::RBRACK(self.tokenmeta()),
            '{' => Token::LBRACE(self.tokenmeta()),
            '}' => Token::RBRACE(self.tokenmeta()),
            ',' => Token::COMMA(self.tokenmeta()),
            '.' => {
                if self.next_is('.') {Token::DBLDOT(self.tokenmeta())}
                else {Token::DOT(self.tokenmeta())}
            },
            ':' => Token::COLON(self.tokenmeta()),
            ';' => Token::SEMICOLON(self.tokenmeta()),
            '+' => Token::PLUS(self.tokenmeta()),
            '-' => Token::MINUS(self.tokenmeta()),
            '*' => {
                if self.next_is('*') {Token::DBLSTAR(self.tokenmeta())}
                else {Token::STAR(self.tokenmeta())}
            },
            '/' => {
                if self.next_is('/') {Token::DBLSLASH(self.tokenmeta())}
                else {Token::SLASH(self.tokenmeta())}
            },
            '%' => Token::PRECENT(self.tokenmeta()),
            '=' => {
                if self.next_is('=') {Token::DBLEQUAL(self.tokenmeta())}
                else if self.next_is('>') {Token::ARROW(self.tokenmeta())}
                else {Token::EQUAL(self.tokenmeta())}
            },
            '>' => {
                if self.next_is('=') {Token::GTEQUAL(self.tokenmeta())}
                else {Token::GREATER(self.tokenmeta())}
            },
            '<' => {
                if self.next_is('=') {Token::LSEQUAL(self.tokenmeta())}
                else {Token::LESS(self.tokenmeta())}
            },
            '!' => {
                if self.next_is('=') {Token::BNGEQUAL(self.tokenmeta())}
                else {Token::BANG(self.tokenmeta())}
            },
            '"' => {
                while self.curr < self.src.len() && self.src[self.curr] != '"' {
                    if self.src[self.curr] == '\n' {return Token::ERROR(self.tokenmeta(), "Unterminated string".to_string())}
                    self.eat();
                }
                if self.curr >= self.src.len() {Token::ERROR(self.tokenmeta(), "Unterminated string".to_string())}
                else {
                    self.eat();
                    Token::STRING(self.tokenmeta(), String::from_iter(&self.src[self.start+1..self.curr-1]))
                }
            }
            '0'..='9' => {
                while self.curr < self.src.len() && self.src[self.curr].is_ascii_digit() {self.eat();}
                if self.curr < self.src.len() - 1 && self.src[self.curr] == '.' && self.src[self.curr+1].is_ascii_digit() {
                    self.eat();
                    while self.curr < self.src.len() && self.src[self.curr].is_ascii_digit() {self.eat();}
                    Token::FLOAT(self.tokenmeta(), String::from_iter(&self.src[self.start..self.curr]).parse().unwrap())
                } else {Token::INT(self.tokenmeta(), String::from_iter(&self.src[self.start..self.curr]).parse().unwrap())}
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while self.curr < self.src.len() && (self.src[self.curr].is_ascii_alphanumeric() || self.src[self.curr] == '_') {self.eat();}
                let word = String::from_iter(&self.src[self.start..self.curr]);
                match word.as_str() {
                    "u1" => Token::U1(self.tokenmeta()),
                    "u8" => Token::U8(self.tokenmeta()),
                    "u16" => Token::U16(self.tokenmeta()),
                    "u32" => Token::U32(self.tokenmeta()),
                    "u64" => Token::U64(self.tokenmeta()),
                    "i8" => Token::I8(self.tokenmeta()),
                    "i16" => Token::I16(self.tokenmeta()),
                    "i32" => Token::I32(self.tokenmeta()),
                    "i64" => Token::I64(self.tokenmeta()),
                    "f32" => Token::F32(self.tokenmeta()),
                    "f64" => Token::F64(self.tokenmeta()),
                    "null" => Token::NULL(self.tokenmeta()),
                    "and" => Token::AND(self.tokenmeta()),
                    "or" => Token::OR(self.tokenmeta()),
                    "dyn" => Token::DYN(self.tokenmeta()),
                    "if" => Token::IF(self.tokenmeta()),
                    "else" => Token::ELSE(self.tokenmeta()),
                    "for" => Token::FOR(self.tokenmeta()),
                    "while" => Token::WHILE(self.tokenmeta()),
                    "var" => Token::VAR(self.tokenmeta()),
                    "func" => Token::FUNC(self.tokenmeta()),
                    "return" => Token::RETURN(self.tokenmeta()),
                    _ => Token::ID(self.tokenmeta(), word)
                }
            }
            other => Token::ERROR(self.tokenmeta(), format!("Unrecognized character: `{other}`"))
        }
    }
    fn skip(&mut self) {
        while self.curr < self.src.len() {
            match self.src[self.curr] {
                ' ' | '\t' | '\r' => {self.eat();},
                '\n' => {
                    self.eat();
                    self.line += 1;
                    self.column = 1;
                }
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
    fn tokenmeta(&self) -> TokenMeta {
        TokenMeta{line: self.line, column: self.column - (self.curr - self.start) as u16, length: (self.curr - self.start) as u16}
    }
}