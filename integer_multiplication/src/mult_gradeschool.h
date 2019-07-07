#include <stdint.h>
#include <string>
#include <vector>

namespace alg {

// Represent integers as vector of 0-9 digits
// from least to most significant digit.
using SplitNumber = std::vector<uint8_t>;

// Parse an input string.  Exit with error if non-numeric
// character is found in input.
SplitNumber parse_input(const std::string& num);

// Compute a partial product.  Generate a SplitNumber
// which is the product of a SplitNumber and a decimal
// digit.
SplitNumber partial_product(SplitNumber x, uint8_t y);

// Generate partial products from x and each digit of y.
std::vector<SplitNumber> generate_partials(const SplitNumber& x,
                                           const SplitNumber& y);

// Add partial products for result SplitNumber.
SplitNumber sum(const std::vector<SplitNumber>& partials);

SplitNumber gradeschool_multiply(const SplitNumber& x, const SplitNumber& y);

std::string to_string(const SplitNumber& n);

}  // alg


