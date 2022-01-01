#include <lexer.h>

namespace monkey {
Lexer::Lexer(std::string input) : input(input), read_position(0) {
  read_char();
}
Lexer::~Lexer() {}

Token Lexer::next_token() const { return {}; }
void Lexer::read_char() {
  if (read_position >= input.size()) {
    ch = 0;
  } else {
    ch = input[read_position];
  }
  position = read_position++;
}
}  // namespace monkey
