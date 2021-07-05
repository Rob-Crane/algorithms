# Median of Two

## Problem
Given two sorted arrays, efficiently locate the median of the combined set.

## Approach

### Observations
* If `N` (size of combined set) is odd, task is to locate element `N/2`. If size of combined set is even, task is to locate `N/2-1` and `N/2`.
* Let `ind(x)` be the index of a number `x` from its input list and `rind(x)` be its "reverse index" (distance from last element). Let `IND(x)` and `RIND(x)` be the index in a (theoretical) combined sorted list.
  * *Note*: For a list of length `l` with element `x`, `rind(x) = l - 1 - ind(x)`.
* Suppose one element drawn from each list, let those be `a` and `b`  s.t. `a>=b`. Then `IND(a) >= ind(a) + ind(b) + 1` (`+1` because `b` must also come before `a`) and similarly, `RIND(b) >= rind(a) + rind(b) + 1`. So, let `MIND(a) = ind(a) + ind(b) + 1` and `MRIND(b) = rind(a) + rind(b) + 1`.
* Elimination Rule: 
  * If `MIND(a) >= N/2`, then can eliminate everything `>ind(a)` in `a`'s list. Equivalently, if `MRIND(b) >= N-1-N/2`, can eliminate everything `<ind(b)` in its list.
  * If `ind(a)`, `ind(b)` chosen so `ind(a) + ind(b) == N/2 - 1` then `MIND(a) == N/2` so elimination rule applies for the list from which `a` was drawn.
  * With this choice of indicies, what about list from which `b` drawn?
```
ind(a)+ind(b)==N/2-1
len(a's list)-1-ind(a) + len(b's list)-1-ind(b) == N-1-N/2
rind(a) + rind(b) == N-1-N/2
rind(a) + rind(b) + 1 == N-N/2
MRIND(b) == N-N/2
# So elimination criterion applies (but in other direction, eliminating lower elements).
```
* Approach is to repeatedly apply elimination rule by selecting elements from each list (so that `ind(a) + ind(b) == N/2-1`), comparing them, and eliminating the elements above or below the selected elements.
* Because of the restriction that `ind(x) + ind(y) == N/2-1`, the choice of one element determines the choice of the other. Furthermore, must ensure that the determined choice falls within range. Suppose `y` index of an element chosen from one list by some scheme and `x` is chosen from other list according to `x=N/2-1-y`. What are the restrictions on choice of `y`?
```
Restriction 1:

To ensure x>=0:
y<=N/2-1

To ensure x <= len(x's list)-1
y>=N/2-len(x's list)
```
* For each list, can ignore any elements at `ind > N/2` since they must fall after `N/2` in sorted ordering. Similarly, can ignore any elements at `rind > N/2+1` since they will fall before `N/2-1`. Therefore, this too creates the restriction on choices for `y`:
```
Restriction 2:
len(y's list)-N/2-2<=y<=N/2
```
* So `Restriction 1` is one element more restrictive on `y`'s upper limit. On its lower limit:
```
[N/2-len(x's list)] - [len(y's list)-N/2-2]
2N/2 -len(x's list) - len(y's list) + 2
N - N + 2
```
* So `Restriction 1` is at most two elements more restrictive on `y`'s lower bound. But as long as `y`'s selection scheme doesn't choose either the bottom two elements or top element, `Restriction 1` is sufficient. This is the case in the scheme described below.

### Half List Elimination
* Want to pick elements `i`, `j` so that `i+j = N/2-1`.
* Strategy is to choose "pivots" from one list which are the middle element of remaining interval and choose complementary `N/2-1` pivot from other list (which will divide it roughtly in half). This strategy will terminate when the smaller list has an interval of length `<=2`.

### Final Step:
* Half-list elimination ends when smaller list interval is length `<=2`. Task is to find the `N/2` value from a length `<=2` list and another list of arbitrary size.
* Want to find the median of the total set from the intervals of possible values. If `k` values have been eliminated from the lower ranges of the two input lists, then the task is to find the `N/2-k`th order statistic from the remaining values.
* This can be accomplished in constant time by starting with the `k`th value from the arbitrary size interval and moving backwards up to two times "inserting" the remaining two elements from the eliminated list. This is is similar to the merge step of Merge Sort but moving backwards up to two times from the `kth` value to the preceding elements or to the elements from the smaller list.
