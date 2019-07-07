#include "mult_gradeschool.h"

#include <algorithm>
#include <iostream>
#include <sstream>

namespace alg {

SplitNumber parse_input(const std::string& num) {
  SplitNumber ret;
  ret.reserve(num.size());
  for (auto rit = num.crbegin(); rit != num.crend(); ++rit) {
    unsigned i = *rit - '0';
    if (i < 0 || i > 9) {
      std::cerr << "invalid string." << std::endl;
      std::exit(1);
    }
    ret.push_back(i);
  }
  return ret;
}

SplitNumber partial_product(SplitNumber x, uint8_t y) {
  if (y == 0) {
    return {0};
  }
  SplitNumber ret;
  uint8_t carryover = 0;
  for (uint8_t xi : x) {
    uint8_t p = xi * y;
    uint8_t pc = p + carryover;
    ret.push_back(pc % 10);
    carryover = pc / 10;
  }
  if (carryover != 0) {
    ret.push_back(carryover);
  }
  if (ret.empty()) {
    ret = {0};
  }
  return ret;
}

std::vector<SplitNumber> generate_partials(const SplitNumber& x,
                                           const SplitNumber& y) {
  std::vector<SplitNumber> ret;
  SplitNumber zeros = {};
  for (uint8_t yi : y) {
    SplitNumber partial = partial_product(x, yi);
    SplitNumber shifted(partial.size() + zeros.size());
    std::copy(zeros.begin(), zeros.end(), shifted.begin());
    std::move(partial.begin(), partial.end(), shifted.begin() + zeros.size());
    ret.push_back(shifted);
    zeros.push_back(0);
  }
  return ret;
}

SplitNumber sum(const std::vector<SplitNumber>& partials) {
  long long carry_sum = 0;
  bool have_place = true;
  long place = 0;
  SplitNumber ret;
  while (carry_sum > 0 || have_place) {
    have_place = false;
    for (const auto& partial : partials) {
      if (place < partial.size()) {
        have_place = true;
        carry_sum += partial[place];
      }
    }
    if (carry_sum > 0) {
      ret.push_back(carry_sum % 10);
      carry_sum /= 10;
    }
    ++place;
  }
  if (ret.empty()) {
    ret = {0};
  }
  return ret;
}

SplitNumber gradeschool_multiply(const SplitNumber& x, const SplitNumber& y) {
  std::vector<SplitNumber> partials = generate_partials(x, y);
  return sum(partials);
}

std::string to_string(const SplitNumber& n) {
    std::ostringstream ss;
    for (auto rit = n.crbegin(); rit != n.crend(); ++rit) {
      ss << int(*rit);
    }
    return ss.str();
}

}  // alg
