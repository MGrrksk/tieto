#include <logger.hpp>
#include <cstdarg>

void Logger::errorAt(const Token& token, const char* msg, ...) {
    errorHappened = true;
    fprintf(errorFile, "\e[1;91m[Error] \e[1;96m%s:%i:%i\e[1;97m - ", codeFilename, token.line, token.collumn);
    va_list args;
    va_start(args, msg);
    vfprintf(errorFile, msg, args);
    va_end(args);
    fprintf(errorFile, "\n\e[0m     |\n%04i |\t\e[1;37m%s", token.line, codeLines[token.line - 1].c_str());
    fprintf(errorFile, "\n\e[0m     |\t%s\e[1;91m%s\e[0m\n", std::string(token.collumn - 1, ' ').c_str(), std::string(token.length, '^').c_str());
}