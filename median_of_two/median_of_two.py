class Interval:
    def __init__(self, nums):
        self.nums = nums
        self.begin_idx = 0
        self.end_idx = len(self.nums)
    
    def at(self, i):
        assert i >= self. begin_idx and i < self.end_idx
        return self.nums[i]
    
    def front(self):
        assert self.begin_idx >= 0 and self.begin_idx < self.end_idx
        return self.nums[self.begin_idx]
    
    def last(self):
        assert self.begin_idx >= 0 and self.end_idx <= len(self.nums)
        return self.nums[self.end_idx-1]
    
    def empty(self):
        assert self.begin_idx <= self.end_idx
        return self.begin_idx == self.end_idx
    
    def eliminate_below(self, i):
        assert i >= self.begin_idx and i < self.end_idx
        self.begin_idx = i
        
    def eliminate_above(self, i):
        assert i >= self.begin_idx and i <= self.end_idx
        self.end_idx = i + 1
        
    def __len__(self):
        assert self.begin_idx <= self.end_idx
        return self.end_idx - self.begin_idx
    
    def __str__(self):
        return str(self.nums[self.begin_idx:self.end_idx])

def increment_down(interval_a, interval_b):
    if interval_a.empty() and interval_b.empty():
        ret = None
    else:
        if interval_a.empty():
            decrement_interval = interval_b
        elif interval_b.empty():
            decrement_interval = interval_a
        elif interval_a.last() > interval_b.last():
            decrement_interval = interval_a
        else:
            decrement_interval = interval_b
        ret = decrement_interval.last()
        decrement_interval.end_idx-=1
    return ret
    

# Find the N/2 and N/2-1 values from two intervals where one is arbitrary length
# and the other is size <= 2. Returns (value at N/2-1, value at N/2).
def find_medians_from_reduced(interval1, interval2, n_2):
    
    # Algorithm is linear in length of shorter interval. Let x be the shorter
    # interval and y the longer.
    if len(interval1) < len(interval2):
        x = interval1
        y = interval2
    else:
        x = interval2
        y = interval1
    
    # t is the index of n/2 in y if no more elements from x preceded it.
    
    m1, m2 = None, None
    
    # From elimination, N/2 and N/2-1 positions must remain in intervals.
    # Case 1 begins by assuming N/2 occurs in y, then "adds" non-eliminated
    # elements from x and tracks the N/2th and N/2-1th positions.
    
    # Won't add eliminated elements from x so compute index of target N/2
    # after adding non-eliminated elements.
    t = n_2 - x.begin_idx
    assert t >= y.begin_idx # Indicate problematic elimination.
    # Case 1: 
    if t < y.end_idx:
        
        # N/2 can't come before t so can eliminate everything above.
        y.end_idx = t+1
        
        # Only "add" elements from x < y[t].
        while not x.empty() and x.last() > y.at(t):
            x.end_idx-=1
        steps_back = len(x) + 2
    # Case 2:
    else:
        steps_back = x.end_idx + y.end_idx - n_2 + 1
        assert steps_back > 0
        
    for _ in range(steps_back):
        m2 = m1
        m1 = increment_down(x, y)
    return m1, m2
        
class Solution:
    
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        
        
        n_2 = (len(nums1) + len(nums2)) // 2
        interval1 = Interval(nums1)
        interval2 = Interval(nums2)
        
        # Works:
        # interval1.begin_idx = max(0, len(nums1) - n_2 - 2)
        # interval2.begin_idx = max(0, len(nums2) - n_2 - 2)
        # interval1.end_idx = min(n_2+1, interval1.end_idx)
        # interval2.end_idx = min(n_2+1, interval2.end_idx)
        
        interval1.begin_idx = max(0, n_2 - len(nums2)-1)
        interval2.begin_idx = max(0, n_2 - len(nums1)-1)
        interval1.end_idx = min(n_2+1, interval1.end_idx)
        interval2.end_idx = min(n_2+1, interval2.end_idx)

        print('b1:',
              interval1.begin_idx,
              'e1:',
              interval1.end_idx,
              'b2:',
              interval2.begin_idx,
              'e2:',
              interval2.end_idx,)
        print(nums1, nums2)
        
        while len(interval1) > 2 and len(interval2) > 2:
            # Save starting interval lengths.
            len1 = len(interval1)
            len2 = len(interval2)
            
            # Choose middle index of interval1.
            idx1 = interval1.begin_idx + (interval1.end_idx - interval1.begin_idx) // 2
            # Choose interval2 index so sum of intervals is N/2 - 1
            idx2 = n_2 - 1 - idx1
            if interval1.at(idx1) < interval2.at(idx2):
                interval1.eliminate_below(idx1) # TODO: is this right?
                interval2.eliminate_above(idx2)
            else:
                interval2.eliminate_below(idx2)
                interval1.eliminate_above(idx1)
                
            # Infinite loop guard.
            assert len1 != len(interval1) or len2 != len(interval2)
            
        m1, m2 = find_medians_from_reduced(interval1, interval2, n_2)
        
        if (len(nums1) + len(nums2)) % 2 == 0:
            return (m1 + m2) / 2
        else:
            return m2