#include <fstream>
#include <logger.hpp>

int main(int argc, const char* argv[]) {
    if (argc != 2) return 1;
    Logger::codeFilename = argv[1];
    std::ifstream file(argv[1]);
    if (!file) return 1;
    std::string line, str;
    while (std::getline(file, line)) {
        Logger::codeLines.push_back(line);
        str += line + '\n';
    }
    Logger::errorAt({6, 17, 3, "fib"}, "Undefined function `%s`", "fib");
    file.close();
    return 0;
}