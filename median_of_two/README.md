# Median of Two

## Problem
Given two sorted arrays, efficiently locate the median of the combined set.

## Approach

### Observations
* If `N` (size of combined set) is odd, task is to locate element `N/2`. If size of combined set is even, task is to locate `N/2-1` and `N/2` so will generalize by focusing on finding the latter.
* Let `ind(x)` be the index of a number `x` from its intput list and `rind(x)` be its "reverse index" (distance from last element). Let `IND(x)` and `RIND(x)` be the index in a (theoretical) combined sorted list.
  * *Note*: For a list of length `l` with element `x`, `rind(x) = l - 1 - ind(x)`.
* Given `a` and `b`, drawn from separate lists, s.t. `a>b`, then `IND(a) >= ind(a) + ind(b) + 1` (`+1` because `b` must also come before `a`) and similarly, `RIND(b) >= rind(a) + rind(b) + 1`. So, let `MIND(a) = ind(a) + ind(b) + 1` and `MRIND(b) = rind(a) + rind(b) + 1`.
* Elimination Rule: 
  * If `MIND(a) >= N/2`, then can eliminate everything `>ind(a)` in `a`'s list. Equivalently, if `MRIND(b) >= RIND(N/2)`, can eliminate everything `<ind(b)` in its list.
  * If `ind(a)`, `ind(b)` chosen so `ind(a) + ind(b) == N/2` then `MIND(a) == N/2 + 1` so elimination rule applies.
  * With this choice of indicies, what can be said about reverse index of `b`?
```
ind(a)+ind(b)==N/2
len(a's list)-1-ind(a) + len(b's list)-1-ind(b) == N-2-N/2
rind(a) + rind(b) == N-2-N/2
rind(a) + rind(b) + 1 == N-1-N/2
MRIND(b) == RIND(N/2)
# So elimination criterion applies.
```

### Half List Elimination
* Want to pick elements `i`, `j` so that `i+j = N/2`.
* Such pairs will exist for every element in the shorter list and all elements in the longer list s.t. `for element i of shorter list N/2-i <= [length of shorter list]`.
* Strategy is to choose "pivots" from smaller list which are the middle element of remaining interval and choose complementary `N/2` pivot from longer list. This strategy will terminate when one of the intervals has length `<=2`.

### Final Step:
* Half-list elimination ends when one of the two intervals has length `<=2`. Task is to find the `N/2` value from a length `<=2` list and another list of arbitrary size.
* Want to find the median of the total set from the intervals of possible values. If `k` values have been eliminated from the lower ranges of the two input lists, then the task is to find the `N/2-k`th order statistic from the intervals.
* This can be accomplished in constant time by starting with the `k`th value from the arbitrary size interval and then examining the one or two values in the other interval to see if they fall before or after the `k`th value of the longer list.
