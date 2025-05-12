#include <fstream>
#include <logger.hpp>
#include <lexer.hpp>

int main(int argc, const char* argv[]) {
    // Read the file
    if (argc != 2) return 1;
    Logger::codeFilename = argv[1];
    std::ifstream file(argv[1]);
    if (!file) return 1;
    std::string line, str;
    while (std::getline(file, line)) {
        Logger::codeLines.push_back(line);
        str += line + '\n';
    }
    file.close();
    // Lexical analysis
    Lexer lexer;
    lexer.setSource(str.c_str());
    Token token;
    do {
        token = lexer.getToken();
        printf("%i - %u:%u `%.*s`[%u]\n", token.type, token.line, token.column, token.length, token.lexeme, token.length);
    } while (token.type != TT_EOF);
    
    return 0;
}