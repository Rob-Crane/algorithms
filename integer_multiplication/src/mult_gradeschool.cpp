#include "mult_gradeschool.h"

#include <algorithm>

namespace alg {

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


SplitNumber gradeschool_multiply(const SplitNumber& x, const SplitNumber& y) {
  std::vector<SplitNumber> partials = generate_partials(x, y);
  return sum(partials);
}


}  // alg
