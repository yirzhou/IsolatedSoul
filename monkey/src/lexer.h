#include <token.h>

#include <memory>
#include <string>

namespace monkey {

class Lexer {
 public:
  Lexer(std::string input);
  ~Lexer();
  Token next_token();
  void read_char();

 private:
  const std::string input;
  int position;
  int read_position;
  char ch;
};

}  // namespace monkey
