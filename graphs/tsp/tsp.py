from argparse import ArgumentParser
from itertools import combinations

from scipy.spatial import KDTree
import numpy as np

def path_to_k(s, j):
    l = list(s)
    l.remove(j)
    return tuple(l)


def _dp_solve(cities):
    """
    Compute an exact solution using dynamic programming in exponential time.
    """
    A = {}
    for m in range(1, len(cities)): # Number of cities in path after city 0.
        for s in combinations(range(1, len(cities)), m): # Cities in path after city 0.
            A[s] = {}
            for j in s: # Last city in path.
                min_len = float('inf')
                to_k = path_to_k(s, j) # Remove j from path.
                if not to_k:
                    A[s][j] = np.linalg.norm(cities[0]-cities[j])
                else:
                    for k in s:
                        if k != j:
                            min_len = min(min_len, A[to_k][k] + np.linalg.norm(cities[k]-cities[j]))
                    A[s][j] = min_len
    last_s = tuple(range(1, len(cities)))
    min_len = float('inf')
    for j, cost in A[last_s].items():
        min_len = min(min_len, cost + np.linalg.norm(cities[j] - cities[0]))
    return min_len

class NeighborFinder:

    def __init__(self, cities):
        self.tree = KDTree(cities)
        self.visited = set([0])
        self.cities = cities
        self.last = 0

    def __iter__(self):
        return self

    def __next__(self):
        if len(self.visited) == len(self.cities):
            raise StopIteration
        k = 2
        while True:
            # Query for the k nearest points.
            d, i = self.tree.query([self.cities[self.last]], k)
            # Ignoring first result (will be last city visited), find
            # first unvisited city in results.
            for c, dist in zip(i[0][1:], d[0][1:]):
                if c not in self.visited:
                    next_city = c
                    self.last = next_city
                    self.visited.add(next_city)
                    return next_city, dist
            k = min(2*k, len(self.cities))



def _greedy_solve(cities):
    """
    Compute an approximate solution using a greedy nearest neighbor heuristic.
    """
    ordering, distances = [], []

    for c, d in NeighborFinder(cities):
        ordering.append(c)
        distances.append(d)
    return sum(distances) + np.linalg.norm(cities[0]-cities[ordering[-1]])

def min_tour_cost(cities, method='dp'):
    if method == 'dp':
        min_len = _dp_solve(cities)
    elif method == 'greedy':
        min_len = _greedy_solve(cities)
    else:
        raise RuntimeError('Unsupported method')
    return int(min_len)


def get_cities(input_file):
    num_cities = None
    cities = []
    with open(input_file) as f:
        for l in f:
            if num_cities is None:
                num_cities = int(l)
            else:
                tokens = l.split()
                if len(tokens) == 2:
                    lat, lon = [float(t) for t in tokens]
                if len(tokens) == 3:
                    _, lat, lon = [float(t) for t in tokens]
                cities.append([lat, lon])
    return np.array(cities)


def parse_args():
    parser = ArgumentParser()
    method_group = parser.add_mutually_exclusive_group(required=True)
    method_group.add_argument('--dp', action='store_true')
    method_group.add_argument('--greedy', action='store_true')
    parser.add_argument('input_file')
    return parser.parse_args()
  
def main():
    args = parse_args()
    cities = get_cities(args.input_file)
    print('Min tour cost:', min_tour_cost(cities, 'dp' if args.dp else 'greedy'))
  

if __name__ == '__main__':
    main()
