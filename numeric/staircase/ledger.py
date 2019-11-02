import sys
def count_ways(n):

    # Base case(s)
    if n == 0:
        return 1
    if n < 0:
        return 0

    # Value already computed.
    if n in ledger:
        return ledger[n]

    # Compute from recursion.
    counts = count_ways(n-1) + count_ways(n-2) + count_ways(n-3)
    ledger[n] = counts
    return counts
print(count_ways(100))
