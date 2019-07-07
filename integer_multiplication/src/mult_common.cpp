#include "mult_common.h"

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

std::string to_string(const SplitNumber& n) {
    std::ostringstream ss;
    for (auto rit = n.crbegin(); rit != n.crend(); ++rit) {
      ss << int(*rit);
    }
    return ss.str();
}

}  // alg
