#[derive(Debug, PartialEq)]
pub enum TokenType {
    // OPERATORS
    LPAREN,
    RPAREN,
    LBRACK,
    RBRACK,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    DBLDOT,
    COLON,
    SEMICOLON,
    QUESTION,
    PLUS,
    MINUS,
    STAR,
    DBLSTAR,
    SLASH,
    DBLSLASH,
    PRECENT,
    EQUAL,
    DBLEQUAL,
    ARROW,
    GREATER,
    GTEQUAL,
    LESS,
    LSEQUAL,
    BANG,
    BNGEQUAL,
    PIPEFORWARD,
    // LITERALS
    INT(i64),
    FLOAT(f64),
    STRING(String),
    ID(String),
    // TYPES
    U1,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    // KEYWORDS
    NULL,
    AND,
    OR,
    DYN,
    IF,
    ELSE,
    FOR,
    WHILE,
    VAR,
    FUNC,
    RETURN,
    // OTHER
    NEWLINE,
    ERROR(String),
    EOF
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub line: u16,
    pub column: u16,
    pub length: u16
}