#include <token.h>

namespace monkey {

TokenType lookup_ident(string ident) {
  if (keywords.find(ident) != keywords.end()) {
    return keywords.at(ident);
  }
  return IDENT_T;
}
}  // namespace monkey
