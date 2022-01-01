#include <string>

namespace monkey {
using namespace std;

typedef string TokenType;

struct Token {
  TokenType Type;
  string Literal;
};

static const TokenType ILLEGAL_T = "ILLEGAL";
static const TokenType EOF_T = "EOF";

// Identifiers + literals
static const TokenType IDENT_T = "IDENT";
static const TokenType INT_T = "INT";

// Operators
static const TokenType ASSIGN_T = "=";
static const TokenType PLUS_T = "+";

// Delimiters
static const TokenType COMMA_T = ",";
static const TokenType SEMICOLON_T = ";";

static const string LPAREN_T = "(";
static const string RPAREN_T = ")";
static const string LBRACE_T = "{";
static const string RBRACE_T = "}";

// Keywords
static const TokenType FUNCTION_T = "FUNCTION";
static const TokenType LET_T = "LET";
}  // namespace monkey
