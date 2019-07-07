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

// Add partial products for result SplitNumber.
SplitNumber sum(const std::vector<SplitNumber>& partials);

std::string to_string(const SplitNumber& n);
}  // alg
