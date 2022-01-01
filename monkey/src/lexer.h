#include <token.h>

#include <memory>
#include <string>

namespace monkey {

class Lexer {
 public:
  Lexer(std::unique_ptr<std::string> input);
  ~Lexer();
  Token next_token() const;

 private:
  std::unique_ptr<std::string> input;
  int position;
  int readPosition;
  char ch;
};

}  // namespace monkey
