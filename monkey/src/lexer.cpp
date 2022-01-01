#include <lexer.h>

namespace monkey {
Lexer::Lexer(std::unique_ptr<std::string> input) : input(std::move(input)) {}
Lexer::~Lexer() {}

Token Lexer::next_token() const { return {}; }
}  // namespace monkey
