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
      "let result = add(five, ten);";

  std::vector<Token> tests{
      {LET_T, "let"},      {IDENT_T, "five"},  {ASSIGN_T, "="},
      {INT_T, "5"},        {SEMICOLON_T, ";"}, {LET_T, "let"},
      {IDENT_T, "ten"},    {ASSIGN_T, "="},    {INT_T, "10"},
      {SEMICOLON_T, ";"},  {LET_T, "let"},     {IDENT_T, "add"},
      {ASSIGN_T, "="},     {FUNCTION_T, "fn"}, {LPAREN_T, "("},
      {IDENT_T, "x"},      {COMMA_T, ","},     {IDENT_T, "y"},
      {RPAREN_T, ")"},     {LBRACE_T, "{"},    {IDENT_T, "x"},
      {PLUS_T, "+"},       {IDENT_T, "y"},     {SEMICOLON_T, ";"},
      {RBRACE_T, "}"},     {SEMICOLON_T, ";"}, {LET_T, "let"},
      {IDENT_T, "result"}, {ASSIGN_T, "="},    {IDENT_T, "add"},
      {LPAREN_T, "("},     {IDENT_T, "five"},  {COMMA_T, ","},
      {IDENT_T, "ten"},    {RPAREN_T, ")"},    {SEMICOLON_T, ";"},
      {EOF_T, ""}};
  Lexer l(std::move(input));

  for (size_t i = 0; i < tests.size(); ++i) {
    Token tok = l.next_token();
    EXPECT_EQ(tok.Type, tests[i].Type);
    EXPECT_EQ(tok.Literal, tests[i].Literal);
  }
}
}  // namespace monkey
