# Magic Squares

## Problem
Given a 3x3 square filled with [1-9], compute the L1 distance to the nearest [magic square](https://en.wikipedia.org/wiki/Magic_square).

## Solution
Approach is to compute all 3x3 magic squares and then loop over them to compute the nearest.  Two observations make computing the magic squares easier:
* The 3x3 "magic constant" is 15.  The sum of ALL numbers in the square is `1 + 2 + 3 + ... + 9 = 45`.  Since each row (or column) must be equal and the sum of the row totals (or column totals) must equal 45, then each row (or column) must sum to 15.
* The middle number is 5 and the square is filled by "opposing pairs" that sum to 10 (like 1 and 9, 3 and 7, etc.).

Armed with the second observation, can construct a recursive algorithm that fills a square with candidate "opposing pairs" and saves any squares that meet the criteria for being magic.
