## Problem
Compute an array `P` where each element is the product of all other elements in the array.  Do not use division.

## Solution
Create an empty results array the size of the input.  In one pass over the input, accumulate the product in the result array.  The first element should be 1.  In a second, reversed, pass over the input, accumulate a product from the end of the array and multiply by the value in the results array.

```
prod = 1
res = []
for n in arr:
    res.append(prod)
    prod*=n


prod = 1
for i, n in enumerate(arr[::-1]):
    res[len(res) - i - 1] *= prod
    prod *= n
return prod
```
