# Problem

Given a staircase of length K, compute number of different ways to reach top taking one, two, or three steps at a time.

## Linear Ledger Solution

The solution is a recurrence relationship where the number of ways `T` at any step `n` is: `T(n) = T(n-1) + T(n-2) + T(n-3)` with the base cases `T(1) = 1` and `T(n < 1) = 0`.  By caching values previously computed, the time complexity is linear.  

## Algebraic Solution
The problem can be described as matrix multiplication:
```
F(n) = | T(n)   |
       | T(n-1) |
       | T(n-2) |

F(n+1) = | 1 1 1 |
         | 1 0 0 | x F(n)
         | 0 1 0 | 

Given:

F(3) = | 4 |
       | 2 |
       | 1 |

F(n) = | 1 1 1 | ^ (n-3)
       | 1 0 0 |          x F(3)
       | 0 1 0 | 
```
Note that the matrix A can be diagnolized (has imaginary eignvalues) to produce a very efficient solution.  Otherwise, the matrix exponentiation can be solved be repeated squaring for `O(log N)` solution.
