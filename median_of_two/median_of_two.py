# from collections import namedtuple

class Interval:
    def __init__(self, values):
        self.values = values
        self.start = 0
        self.end = len(self.values)

    def __getitem__(self, idx):
        assert idx >= self.start and idx < self.end
        return self.values[idx]

    def __len__(self):
        return self.end - self.start

# """
# Container for results of half-list elimination when algorithm terminated.
# Contains an arbitrary length instance of Interval corresponding to eliminated
# values from one input and the one (x) or two (x, y) values remaining from other
# input.
# """
# ReducedIntervals = namedtuple('ReducedIntervals', 'interval x y'])

def get_reduced_intervals(values_i,
                          values_j):
    """
    Iteratively reduce median possibility intervals until of the intervals
    is length <= 2
    Returns:
      interval_short, interval_long where len(interval_short)<=2
    """
    N_2 = (len(values_i) + len(values_j)) // 2
    
    if len(values_i) <= len(values_j):
        pivot_interval = Interval(values_i)
        npivot_interval = Interval(values_j)
    else:
        pivot_interval = Interval(values_j)
        npivot_interval = Interval(values_i)

    while len(pivot_interval) >= 2:
        pivot_idx = pivot_interval.start + len(pivot_interval) // 2
        assert pivot_idx < pivot_interval.end
        npivot_idx = N_2 - pivot_idx
        assert npivot_idx < npivot_interval.end

        if pivot_interval[pivot_idx] < npivot_interval[npivot_idx]:
            pass
        else:
            pass

        if len(npivot_interval) < len(pivot_interval):
            pivot_interval, npivot_interval = npivot_interval, pivot_interval

    return pivot_interval, npivot_interval


def find_from_reduced(pivot_interval, npivot_interval):
    """
    Find the median from intervals where shorter is length <= 2.
    """
    pass


def find_median(values_i, values_j):
    return find_from_reduced(*get_reduced_intervals(values_i, values_j))
