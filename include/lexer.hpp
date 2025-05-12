#pragma once
#include <token.hpp>

class Lexer {
    const char* src;
    char *start, *curr;
    int line, column;
    bool isEOF();
    char eat();
    bool nextIs(char c);
    Token token(TokenType type);
    bool isKeyword(const char* rest);
    void skip();
    public:
        void setSource(const char* src);
        Token getToken();
};