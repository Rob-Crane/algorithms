"""
Determine whether a string A of upper and lower case
letters can be transformed into a string B of upper case letters
with the following allowed transformations:
  - Lower case letters of A can be deleted.
  - Lower case letters of A can be made upper case.

Solution considers a prefix of B at each iteration.  Each
iteration produces a set of indices of A, each of which
is a valid index to be the "last" character of A in a match.
The next interation considers a new character of B.
All the previous terminating locations are considered and
new possible terminating positions in A are:
  - The next uppercase letter if it matches the new B char.
  - Any 'matching' lowercase letter that occur before the next
    upper case letter.
Finally the algorithm must check for any 'trailing'
upper case letters in A.  Consider the longest prefix
of A that can transformed into B.  If any upper case
characters occur after this prefix, then A cannot
be transformed.
"""
def is_caps(x):
    return x >= 'A' and x <= 'Z'

def set_nxt(ind, A, b, nxt):
    """
    Add "terminating" indices to nxt at
    locations >= ind.  Stop when first
    upper case letter encountered (or
    end of A reached).
    """
    while ind < len(A):
        if is_caps(A[ind]):
            if A[ind] == b:
                nxt.add(ind)
            break
        if A[ind].upper() == b:
            nxt.add(ind)
        ind += 1
    
def abbreviation(A, B):
    """
    Determine whether A can be x-formed into B.
    """
    curr = None
    for b in B:
        nxt = set()
        if not curr:
            set_nxt(0, A, b, nxt)
        else:
            for a_i in curr:
                set_nxt(a_i+1, A, b, nxt)
        curr = set(nxt)
    if not curr:
        # No prefix of A can be x-formed.
        return False
    last_t = max(curr)
    if any(is_caps(a) for a in A[last_t+1:]):
        # Trailing upper case characters in A.
        return False
    return True
