# Rolling Median

Given a stream of numbers, compute the median of the last k numbers.

## Solution

Maintain two data structures:
1. A sorted tree like a `std::multiset`.
2. A queue of references to nodes of the tree.

To update the tree, insert the new element, update the median, remove the oldest element, and finally update the median again.  For the element _n_ addition to tree:
* If _numElements_ is even and _n_ < _median_, decrement median.
* If _numElements_ is odd and _n_ => _median_, increment median.
Then immediately prior to the removal of element _k_:
* If _numElements_ is even and _k_ <= _median_, increment median.
* If _numElements_ is odd and _k_ > _median_, decrement median.

This update scheme assumes that equal element are added as the last element in the range (of equally valued elements) so that the oldest element in that range will be the first in that range.  (true for `std::multiset`).

A potentially easier approach which doesn't scale well for large ranges of input values is to use a hashtable of element frequencies.  One nice simplification is that the median could point to the value mapped to a count rather than a single element node.
