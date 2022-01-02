#include <lexer.h>

namespace monkey {
Lexer::Lexer(std::string input) : input(input), read_position(0) {
  read_char();
}
Lexer::~Lexer() {}

Token Lexer::next_token() {
  Token tok;

  switch (ch) {
    case '=':
      tok.Type = ASSIGN_T;
      break;
    case ';':
      tok.Type = SEMICOLON_T;
      break;
    case '(':
      tok.Type = LPAREN_T;
      break;
    case ')':
      tok.Type = RPAREN_T;
      break;
    case ',':
      tok.Type = COMMA_T;
      break;
    case '+':
      tok.Type = PLUS_T;
      break;
    case '{':
      tok.Type = LBRACE_T;
      break;
    case '}':
      tok.Type = RBRACE_T;
      break;
    case 0:
      tok.Type = EOF_T;
      break;
  };

  tok.Literal = ch == 0 ? "" : std::string(1, ch);
  read_char();
  return tok;
}
void Lexer::read_char() {
  if (read_position >= input.size()) {
    ch = 0;
  } else {
    ch = input[read_position];
  }
  position = read_position++;
}

}  // namespace monkey
