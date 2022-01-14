#include <string>
#include <unordered_map>

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
static const TokenType MINUS_T = "-";
static const TokenType BANG_T = "!";
static const TokenType ASTERISK_T = "*";
static const TokenType SLASH_T = "/";
static const TokenType LT_T = "<";
static const TokenType GT_T = ">";

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
static const TokenType TRUE_T = "TRUE";
static const TokenType FALSE_T = "FALSE";
static const TokenType IF_T = "IF";
static const TokenType ELSE_T = "ELSE";
static const TokenType RETURN_T = "RETURN";

static const TokenType EQ_T = "==";
static const TokenType NOT_EQ_T = "!=";

static const unordered_map<string, TokenType> keywords{
    {"fn", FUNCTION_T},  {"let", LET_T}, {"true", TRUE_T},
    {"false", FALSE_T},  {"if", IF_T},   {"else", ELSE_T},
    {"return", RETURN_T}};

TokenType lookup_ident(string ident);
}  // namespace monkey
