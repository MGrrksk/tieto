#pragma once
#include <token.hpp>
#include <vector>
#include <string>

struct Logger {
    inline static FILE* outputFile = stdout;
    inline static FILE* errorFile = stderr;
    inline static bool errorHappened = false;
    inline static const char* codeFilename;
    inline static std::vector<std::string> codeLines;
    static void errorAt(const Token& token, const char* msg, ...);
};