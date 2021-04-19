# Median of Two

## Problem
Given two sorted arrays, efficiently locate the median of the combined set.

## Approach

### Observations
* If `N` (size of combined set) is odd, task is to locate element `N/2`. If size of combined set is even, task is to locate `N/2-1` and `N/2` so will generalize for latter.
* Assume distinct numbers.
* Let `ind(x)` be the index of a number `x` from its intput list and `rind(x)` be its "reverse index" (distance from last element). Let `IND(x)` and `RIND(x)` be the index in a (theoretical) combined sorted list.
* Given `a` and `b` s.t. `a>b`, then `IND(a) >= ind(a) + ind(b)` and similarly, `RIND(b) >= rind(a) + rind(b)`.
* Elimination Rule: 
