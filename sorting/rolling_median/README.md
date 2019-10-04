# Rolling Median

Given a stream of numbers, compute the median of the last k numbers.

## Solution

Maintain two data structures:
1. A sorted tree like a `std::multiset`.
2. A queue of references to nodes of the tree.

Maintain a reference to the current median of sorted values in tree.
