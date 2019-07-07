#define CATCH_CONFIG_MAIN
#include "catch.hpp"

#include "mult_gradeschool.h"

using namespace alg;

TEST_CASE("partial product check") {
  SplitNumber x{4, 3, 2, 1};
  uint8_t y = 5;
  SplitNumber p = partial_product(x, y);
  SplitNumber exp = {0, 7, 1, 6};
  REQUIRE(p == exp);
  x = {9, 9, 9, 9};
  y = 9;
  p = partial_product(x, y);
  exp = {1, 9, 9, 9, 8};
  REQUIRE(p == exp);
  y = 0;
  p = partial_product(x, y);
  exp = {0};
  REQUIRE(p == exp);
}

TEST_CASE("partials generate check") {
  SplitNumber x = {8, 7, 6, 5};
  SplitNumber y = {4, 3, 2, 1};
  std::vector<SplitNumber> res = generate_partials(x, y);
  std::vector<SplitNumber> exp = {{2, 1, 7, 2, 2},
                                  {0, 4, 3, 0, 7, 1},
                                  {0, 0, 6, 5, 3, 1, 1},
                                  {0, 0, 0, 8, 7, 6, 5}};
  REQUIRE(res == exp);
}

TEST_CASE("test gradeschool") {
  SplitNumber x = parse_input(
      "3141592653589793238462643383279502884197169399375105820974944592");
  SplitNumber y = parse_input(
      "2718281828459045235360287471352662497757247093699959574966967627");

  std::string prd = to_string(gradeschool_multiply(x, y));
  std::string exp =
      "853973422267356706546355086954657449503488853576511496187960112706774304"
      "4893204848617875072216249073013374895871952806582723184";
  REQUIRE(prd == exp);
}
