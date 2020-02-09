from argparse import ArgumentParser
from enum import Enum

class PriorityScheme(Enum):
    DIFFERENCE = 1
    RATIO = 2

class Job:

    def __init__(self, weight, length, priority_scheme):
        self.weight = weight
        self.length = length
        self.difference = weight - length
        self.ratio = weight / length
        self.priority_scheme = priority_scheme

    def __lt__(self, other):
        if self.priority_scheme == PriorityScheme.DIFFERENCE:
            priority = self.difference
            other_priority = other.difference
        elif self.priority_scheme == PriorityScheme.RATIO:
            priority = self.ratio
            other_priority = other.ratio
        else:
            raise RuntimeError('Unsupported priority scheme.')

        if priority == other_priority:
            return self.weight < other.weight
        return priority < other_priority

def compute_total_cost(jobs):
    sorted_jobs = sorted(jobs, reverse=True)
    completion_time = 0.0
    def get_weighted_cost(job):
        nonlocal completion_time
        completion_time += job.length
        return completion_time * job.weight
    return sum(get_weighted_cost(j) for j in sorted_jobs)


def parse_args():
    parser = ArgumentParser()
    parser.add_argument('jobs_file')
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--difference', action='store_true')
    group.add_argument('--ratio', action='store_true')
    args = parser.parse_args()
    return args


def main():
    args = parse_args()
    num_jobs = None
    scheme = PriorityScheme.DIFFERENCE if args.difference else PriorityScheme.RATIO
    jobs = []
    for l in open(args.jobs_file):
        if num_jobs is None:
            num_jobs = int(l)
        else:
            [w, l] = l.split(' ')
            jobs.append(Job(float(w), float(l), scheme))
    assert len(jobs) == num_jobs
    print('Weighted sum: {}'.format(compute_total_cost(jobs)))

if __name__ == '__main__':
    main()
