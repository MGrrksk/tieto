#[derive(Debug, PartialEq)]
pub struct TokenMeta {
    pub line: u16,
    pub column: u16,
    pub length: u16
}

#[derive(Debug, PartialEq)]
pub enum Token {
    // OPERATORS
    LPAREN(TokenMeta),
    RPAREN(TokenMeta),
    LBRACK(TokenMeta),
    RBRACK(TokenMeta),
    LBRACE(TokenMeta),
    RBRACE(TokenMeta),
    COMMA(TokenMeta),
    DOT(TokenMeta),
    DBLDOT(TokenMeta),
    COLON(TokenMeta),
    SEMICOLON(TokenMeta),
    PLUS(TokenMeta),
    MINUS(TokenMeta),
    STAR(TokenMeta),
    DBLSTAR(TokenMeta),
    SLASH(TokenMeta),
    DBLSLASH(TokenMeta),
    PRECENT(TokenMeta),
    EQUAL(TokenMeta),
    DBLEQUAL(TokenMeta),
    ARROW(TokenMeta),
    GREATER(TokenMeta),
    GTEQUAL(TokenMeta),
    LESS(TokenMeta),
    LSEQUAL(TokenMeta),
    BANG(TokenMeta),
    BNGEQUAL(TokenMeta),
    // LITERALS
    INT(TokenMeta, i64),
    FLOAT(TokenMeta, f64),
    STRING(TokenMeta, String),
    ID(TokenMeta, String),
    // TYPES
    U1(TokenMeta),
    U8(TokenMeta),
    U16(TokenMeta),
    U32(TokenMeta),
    U64(TokenMeta),
    I8(TokenMeta),
    I16(TokenMeta),
    I32(TokenMeta),
    I64(TokenMeta),
    F32(TokenMeta),
    F64(TokenMeta),
    // KEYWORDS
    NULL(TokenMeta),
    AND(TokenMeta),
    OR(TokenMeta),
    DYN(TokenMeta),
    IF(TokenMeta),
    ELSE(TokenMeta),
    FOR(TokenMeta),
    WHILE(TokenMeta),
    VAR(TokenMeta),
    FUNC(TokenMeta),
    RETURN(TokenMeta),
    // SPECIAL
    ERROR(TokenMeta, String),
    EOF(TokenMeta)
}