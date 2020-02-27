from argparse import ArgumentParser
import heapq
from random import gauss
import sys

def get_huffman_depth(weights):
  cost_heap = [(v, i) for (i, v) in enumerate(weights)]
  heapq.heapify(cost_heap)
  while len(cost_heap) > 1:

    c1, v1 = heapq.heappop(cost_heap)
    c2, v2 = heapq.heappop(cost_heap)
    heapq.heappush(cost_heap, (c1+c2+gauss(0,1E-6), (v1,v2)))
  root = hash(cost_heap[0][1])
  children = {}
  while cost_heap:
    _, v1v2 = cost_heap.pop()
    c1, c2 = v1v2
    children[hash(v1v2)] = (hash(c1), hash(c2))
    if type(c1) == tuple:
        cost_heap.append((None, c1))
    if type(c2) == tuple:
        cost_heap.append((None, c2))

  to_visit = [(0, root)]
  max_depth = 0
  min_leaf = sys.maxsize
  while to_visit:
    n_depth, n = to_visit.pop()
    if n in children:
      c1, c2 = children[n]
      max_depth = max(max_depth, n_depth+1)
      to_visit.append((n_depth+1, c1))
      to_visit.append((n_depth+1, c2))
  else:
      # Leaf.
      min_leaf = min(n_depth, min_leaf)
  return min_leaf, max_depth

def parse_args():
    parser = ArgumentParser()
    parser.add_argument('input')
    return parser.parse_args()

def main():
    args = parse_args()
    with open(args.input) as f:
        lines = f.readlines()
        weights = [int(v) for v in lines[1:]]
    print('min_leaf, max_depth', get_huffman_depth(weights))

if __name__=='__main__':
    main()
