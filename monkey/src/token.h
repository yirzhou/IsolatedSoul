#include <string>

namespace monkey {
using namespace std;

static const string ILLEGAL_T = "ILLEGAL";
static const string EOF_T = "EOF";

// Identifiers + literals
static const string IDENT_T = "IDENT";
static const string INT_T = "INT";

// Operators
static const string ASSIGN_T = "=";
static const string PLUS_T = "+";

// Delimiters
static const string COMMA_T = ",";
static const string SEMICOLON_T = ";";

static const string LPAREN_T = "(";
static const string RPAREN_T = ")";
static const string LBRACE_T = "{";
static const string RBRACE_T = "}";

// Keywords
static const string FUNCTION_T = "FUNCTION";
static const string LET_T = "LET";
}  // namespace monkey
