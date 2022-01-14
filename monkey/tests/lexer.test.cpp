#include <gtest/gtest.h>
#include <lexer.h>

#include <vector>

namespace monkey {

TEST(NextToken, BasicAssertions) {
  std::string input =
      "let five = 5;"
      "let ten = 10;"
      "let add = fn(x, y) {"
      "  x + y;"
      "};"
      "let result = add(five, ten);"
      "!-/*5;"
      "5 < 10 > 5;"
      "if (5 < 10) {"
      "  return true;"
      "} else {"
      "  return false;"
      "}"
      "10 == 10;"
      "10 != 9;";

  std::vector<Token> tests{
      {LET_T, "let"},      {IDENT_T, "five"},    {ASSIGN_T, "="},
      {INT_T, "5"},        {SEMICOLON_T, ";"},   {LET_T, "let"},
      {IDENT_T, "ten"},    {ASSIGN_T, "="},      {INT_T, "10"},
      {SEMICOLON_T, ";"},  {LET_T, "let"},       {IDENT_T, "add"},
      {ASSIGN_T, "="},     {FUNCTION_T, "fn"},   {LPAREN_T, "("},
      {IDENT_T, "x"},      {COMMA_T, ","},       {IDENT_T, "y"},
      {RPAREN_T, ")"},     {LBRACE_T, "{"},      {IDENT_T, "x"},
      {PLUS_T, "+"},       {IDENT_T, "y"},       {SEMICOLON_T, ";"},
      {RBRACE_T, "}"},     {SEMICOLON_T, ";"},   {LET_T, "let"},
      {IDENT_T, "result"}, {ASSIGN_T, "="},      {IDENT_T, "add"},
      {LPAREN_T, "("},     {IDENT_T, "five"},    {COMMA_T, ","},
      {IDENT_T, "ten"},    {RPAREN_T, ")"},      {SEMICOLON_T, ";"},
      {BANG_T, "!"},       {MINUS_T, "-"},       {SLASH_T, "/"},
      {ASTERISK_T, "*"},   {INT_T, "5"},         {SEMICOLON_T, ";"},
      {INT_T, "5"},        {LT_T, "<"},          {INT_T, "10"},
      {GT_T, ">"},         {INT_T, "5"},         {SEMICOLON_T, ";"},
      {IF_T, "if"},        {LPAREN_T, "("},      {INT_T, "5"},
      {LT_T, "<"},         {INT_T, "10"},        {RPAREN_T, ")"},
      {LBRACE_T, "{"},     {RETURN_T, "return"}, {TRUE_T, "true"},
      {SEMICOLON_T, ";"},  {RBRACE_T, "}"},      {ELSE_T, "else"},
      {LBRACE_T, "{"},     {RETURN_T, "return"}, {FALSE_T, "false"},
      {SEMICOLON_T, ";"},  {RBRACE_T, "}"},      {EOF_T, ""}};
  Lexer l(std::move(input));

  for (size_t i = 0; i < tests.size(); ++i) {
    Token tok = l.next_token();
    EXPECT_EQ(tok.Type, tests[i].Type);
    EXPECT_EQ(tok.Literal, tests[i].Literal);
  }
}
}  // namespace monkey
