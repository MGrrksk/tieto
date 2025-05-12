#pragma once

enum TokenType {
    // Operators
    TT_LPAREN,
    TT_RPAREN,
    TT_LBRACK,
    TT_RBRACK,
    TT_LBRACE,
    TT_RBRACE,
    TT_COLON,
    TT_SEMICOLON,
    TT_COMMA,
    TT_DOT,
    TT_PLUS,
    TT_MINUS,
    TT_STAR,
    TT_DBLSTAR,
    TT_SLASH,
    TT_DBLSLASH,
    TT_PRECENT,
    TT_DOLLAR,
    TT_EQUAL,
    TT_DBLEQUAL,
    TT_GREATER,
    TT_GTEQUAL,
    TT_LESS,
    TT_LSEQUAL,
    TT_ARROW,
    TT_LSGT,
    // Literals
    TT_STRING,
    TT_INT,
    TT_FLOAT,
    TT_ID,
    // Keywords
    TT_AND,
    TT_OR,
    TT_XOR,
    TT_NOT,
    TT_TRUE,
    TT_FALSE,
    TT_NULL,
    TT_IF,
    TT_ELSE,
    TT_WHILE,
    TT_FOR,
    TT_VAR,
    TT_CONST,
    TT_FUNC,
    TT_RETURN,
    // Datatypes
    TT_BIT,
    TT_U8,
    TT_I8,
    TT_U16,
    TT_I16,
    TT_U32,
    TT_I32,
    TT_F32,
    TT_U64,
    TT_I64,
    TT_F64,
    TT_DYN,
    // Special
    TT_ERROR,
    TT_EOF
};

struct Token {
    TokenType type;
    unsigned short line;
    unsigned short column;
    unsigned short length;
    const char* lexeme;
};