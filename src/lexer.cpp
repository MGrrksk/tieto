#include <lexer.hpp>
#include <cstring>
#include <cctype>

///////////////////////
// PRIVATE FUNCTIONS //
///////////////////////

bool Lexer::isEOF() {
    return *curr == '\0';
}
char Lexer::eat() {
    ++column;
    return *curr++;
}
bool Lexer::nextIs(char c) {
    if (*curr == c) {
        eat();
        return true;
    } else return false;
}
Token Lexer::token(TokenType type) {
    return {type, line, column - (curr - start), curr - start, start};
}
bool Lexer::isKeyword(const char* rest) {
    return curr - start == strlen(rest) + 1 && memcmp(start + 1, rest, strlen(rest)) == 0;
}
void Lexer::skip() {
    while (!isEOF()) {
        switch (*curr) {
            case ' ':
            case '\t':
                eat();
                break;
            case '\r':
                ++curr;
                break;
            case '\n':
                eat();
                ++line;
                column = 1;
                break;
            case '#':
                while (!isEOF() && *curr != '\n') eat();
                break;
            default: return;
        }
    }
    return;
}

//////////////////////
// PUBLIC FUNCTIONS //
//////////////////////

void Lexer::setSource(const char* src) {
    this->src = start = curr = (char*)src;
    line = column = 1;
}
Token Lexer::getToken() {
    skip();
    start = curr;
    if (isEOF()) return {TT_EOF, line, column, 1, nullptr};
    switch (eat()) {
        case '(': return token(TT_LPAREN);
        case ')': return token(TT_RPAREN);
        case '[': return token(TT_LBRACK);
        case ']': return token(TT_RBRACK);
        case '{': return token(TT_LBRACE);
        case '}': return token(TT_RBRACE);
        case ':': return token(TT_COLON);
        case ';': return token(TT_SEMICOLON);
        case ',': return token(TT_COMMA);
        case '.': return token(TT_DOT);
        case '+': return token(TT_PLUS);
        case '-': return token(TT_MINUS);
        case '%': return token(TT_PRECENT);
        case '$': return token(TT_DOLLAR);
        case '*':
            if (nextIs('*')) return token(TT_DBLSTAR);
            else return token(TT_STAR);
        case '/':
            if (nextIs('/')) return token(TT_DBLSLASH);
            else return token(TT_SLASH);
        case '=':
            if (nextIs('=')) return token(TT_DBLEQUAL);
            else if (nextIs('>')) return token(TT_ARROW);
            else return token(TT_EQUAL);
        case '>':
            if (nextIs('=')) return token(TT_GTEQUAL);
            else return token(TT_GREATER);
        case '<':
            if (nextIs('=')) return token(TT_LSEQUAL);
            else if (nextIs('>')) return token(TT_LSGT);
            else return token(TT_LESS);
        case '"':
            while (!isEOF() && *curr != '"') {
                if (*curr == '\n') return {TT_ERROR, line, column - (curr - start), curr - start, "Unterminated string"};
                eat();
            }
            if (isEOF()) return {TT_ERROR, line, column - (curr - start), curr - start, "Unterminated string"};
            eat();
            return token(TT_STRING);
        default: {
            char c = *start;
            if (isalpha(c) || c == '_') {
                while (isalnum(*curr) || *curr == '_') eat();
                switch (c) {
                    case 'a': if (isKeyword("nd")) return token(TT_AND); break;
                    case 'c': if (isKeyword("onst")) return token(TT_CONST); break;
                    case 'e': if (isKeyword("lse")) return token(TT_ELSE); break;
                    case 'f':
                        if (isKeyword("32")) return token(TT_F32);
                        else if (isKeyword("64")) return token(TT_F64);
                        else if (isKeyword("alse")) return token(TT_FALSE);
                        else if (isKeyword("or")) return token(TT_FOR);
                        else if (isKeyword("unc")) return token(TT_FUNC);
                        break;
                    case 'i':
                        if (isKeyword("8")) return token(TT_I8);
                        if (isKeyword("16")) return token(TT_I16);
                        if (isKeyword("32")) return token(TT_I32);
                        if (isKeyword("64")) return token(TT_I64);
                        if (isKeyword("f")) return token(TT_IF);
                        break;
                    case 'n':
                        if (isKeyword("ot")) return token(TT_NOT);
                        else if (isKeyword("ull")) return token(TT_NULL);
                        break;
                    case 'o': if (isKeyword("r")) return token(TT_OR); break;
                    case 'r': if (isKeyword("eturn")) return token(TT_RETURN); break;
                    case 't': if (isKeyword("rue")) return token(TT_TRUE); break;
                    case 'u':
                        if (isKeyword("8")) return token(TT_U8);
                        if (isKeyword("16")) return token(TT_U16);
                        if (isKeyword("32")) return token(TT_U32);
                        if (isKeyword("64")) return token(TT_U64);
                        break;
                    case 'v': if (isKeyword("ar")) return token(TT_VAR); break;
                    case 'w': if (isKeyword("hile")) return token(TT_WHILE); break;
                    case 'x': if (isKeyword("or")) return token(TT_XOR); break;
                    default: break;
                }
                return token(TT_ID);
            } else if (isdigit(c)) {
                bool (*isrdigit)(char digit);
                if (c == '0')
                    switch (*curr) {
                        case 'b':
                            isrdigit = [](char digit) {return digit == '0' || digit == '1';};
                            eat();
                            break;
                        case 'o':
                            isrdigit = [](char digit) {return digit >= '0' && digit <= '7';};
                            eat();
                            break;
                        case 'x':
                            isrdigit = [](char digit) {return (digit >= '0' && digit <= '9') || (digit >= 'a' && digit <= 'f') || (digit >= 'A' && digit <= 'F');};
                            eat();
                            break;
                        default: isrdigit = [](char digit) {return (bool)isdigit(digit);}; break;
                    }
                else isrdigit = [](char digit) {return (bool)isdigit(digit);};
                while (!isEOF() && isrdigit(*curr)) eat();
                if (*curr == '.' && isrdigit(*(curr + 1))) {
                    eat();
                    while (!isEOF() && isrdigit(*curr)) eat();
                    return token(TT_FLOAT);
                } else return token(TT_INT);
            } else return {TT_ERROR, line, column - (curr - start), curr - start, "Unrecognized character"};
            break;
        }
    }
    return {TT_ERROR, line, column - (curr - start), curr - start, "Unexpected error"};
}