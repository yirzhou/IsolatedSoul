#include <token.h>

#include <memory>
#include <string>

namespace monkey {

class Lexer {
 public:
  Lexer(std::string input);
  ~Lexer();
  Token next_token();

 private:
  const std::string input;
  int position;
  int read_position;
  char ch;

  bool is_letter(char ch) const;
  bool is_digit(char ch) const;
  void read_char();
  char peek_char() const;
  std::string read_identifier();
  std::string read_number();
  void skip_whitespace();
};

}  // namespace monkey
