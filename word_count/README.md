# Word Count Engine

## Problem
Given a document of words and punctuation, generate a sorted list of pairs `(count, word)`.  Words should be stripped of punctuation and converted to lowercase.  The returned list should be sorted first be descending order of count and second by the order the word first appears in the original document.

## Solution
The first step is generating a _token_:_count_ mapping (in an unordered-mapping/hash-table) which can be accomplished in a single pass.  With this accomplished, the task which remains is to sort into the correct ordering.

The general approach will be to create _count_:_token-list_ map, iterate over the input again, and add a token to its respective count-bucket when it is seen for the first time.

One option is to use an associative container which maintains ordering like a BST.  The depth of the BST will be `O(log(k))` where `k` is the number of unique counts.  The sum of the counts is N so in the worst case `N = 1 + 2 + 3 + ... + n_k = k(k-1)/2`.  So k is `O(sqrt(N))`.  Therefore, iterative over the original input and building _token-lists_ for every count is `O(n log(sqrt(n)))`.

Another option is recognizing that the maximum key value in the _count_:_token-list_ associative container is bounded by `N` (if the document is one word repeated).  Therefore, a vector-map container is possible where each index in a random-access list is a possible count-value.  The benefit of a vector-map container in this context is that insertions are constant time _and_ order is preserved.  Building _token-lists_ is `O(n)` and the answer is recovered by a backwards pass over the vector-map.

A final option is to build the _count_:_token-list_ map in a regular hash-table in `O(N)` time.  Then sort the table entries by _count_.  As described above, the number of unique _count_ keys is `O(sqrt(N))` so this final sort is accomplished in `O(sqrt(N) log (sqrt(N)))` (which grows slower than `O(N)` but this solution is still `O(N)` from the earlier steps).
