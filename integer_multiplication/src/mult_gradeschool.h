#include "mult_common.h"

namespace alg {

// Compute a partial product.  Generate a SplitNumber
// which is the product of a SplitNumber and a decimal
// digit.
SplitNumber partial_product(SplitNumber x, uint8_t y);

// Generate partial products from x and each digit of y.
std::vector<SplitNumber> generate_partials(const SplitNumber& x,
                                           const SplitNumber& y);

SplitNumber gradeschool_multiply(const SplitNumber& x, const SplitNumber& y);

}  // alg
