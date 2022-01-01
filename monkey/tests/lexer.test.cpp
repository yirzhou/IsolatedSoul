#include <gtest/gtest.h>
#include <lexer.h>

#include <cstdio>

namespace monkey {

TEST(NextToken, BasicAssertions) {
  std::unique_ptr<std::string> input =
      std::make_unique<std::string>("=+(){},;");

  const int tests_len = 9;
  Token tests[]{{ASSIGN_T, "="}, {PLUS_T, "+"},      {LPAREN_T, "("},
                {RPAREN_T, ")"}, {LBRACE_T, "{"},    {RBRACE_T, "}"},
                {COMMA_T, ","},  {SEMICOLON_T, ";"}, {EOF_T, ""}};
  Lexer l(std::move(input));

  for (size_t i = 0; i < tests_len; ++i) {
    Token tok = l.next_token();
    EXPECT_EQ(tok.Type, tests[i].Type);
    EXPECT_EQ(tok.Literal, tests[i].Literal);
  }
}
}  // namespace monkey
