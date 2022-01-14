#include <lexer.h>

namespace monkey {
Lexer::Lexer(std::string input) : input(input), read_position(0) {
  read_char();
}
Lexer::~Lexer() {}

Token Lexer::next_token() {
  Token tok;
  skip_whitespace();

  switch (ch) {
    case '=':
      if (peek_char() == '=') {
        char c = ch;
        read_char();
        tok.Type = EQ_T;
        tok.Literal = std::string(1, c) + std::string(1, ch);
      } else {
        tok.Type = ASSIGN_T;
      }
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
    case '-':
      tok.Type = MINUS_T;
      break;
    case '!':
      tok.Type = BANG_T;
      break;
    case '/':
      tok.Type = SLASH_T;
      break;
    case '*':
      tok.Type = ASTERISK_T;
      break;
    case '<':
      tok.Type = LT_T;
      break;
    case '>':
      tok.Type = GT_T;
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
    default:
      if (is_letter(ch)) {
        tok.Literal = read_identifier();
        tok.Type = lookup_ident(tok.Literal);
        return tok;
      } else if (is_digit(ch)) {
        tok.Type = INT_T;
        tok.Literal = read_number();
        return tok;
      } else {
        tok = Token{ILLEGAL_T, std::string(1, ch)};
      }
  };

  tok.Literal = ch == 0 ? "" : std::string(1, ch);
  read_char();
  return tok;
}

void Lexer::read_char() {
  if (read_position >= input.size()) {
    ch = '\0';
  } else {
    ch = input[read_position];
  }
  position = read_position;
  read_position++;
}

char Lexer::peek_char() const {
  if (read_position >= input.size()) {
    return '\0';
  } else {
    return input[read_position];
  }
}

std::string Lexer::read_number() {
  int pos = this->position;

  while (is_digit(ch)) read_char();
  return this->input.substr(pos, this->position - pos);
}

void Lexer::skip_whitespace() {
  while (ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r') read_char();
}

bool Lexer::is_letter(char ch) const {
  return ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_';
}

bool Lexer::is_digit(char ch) const { return '0' <= ch && ch <= '9'; }

std::string Lexer::read_identifier() {
  int pos = position;
  while (is_letter(ch)) read_char();
  return input.substr(pos, position - pos);
}

}  // namespace monkey
