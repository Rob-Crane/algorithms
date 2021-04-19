from argparse import ArgumentParser

import numpy as np

def get_edges(input_file):
  with open(input_file) as f:
    num_v, num_e = None, None
    edges = None
    for l in f:
      if edges is None:
        num_v, num_e = [int(t) for t in l.split()]
        edges = np.full((num_v, num_v), float('inf'))
      else:
        i, j, cost = [int(t) for t in l.split()]
        edges[i-1, j-1] = cost
  for i in range(num_v):
    edges[i, i] = 0
  return edges

def get_shortest_path(edges):
  num_v = len(edges)
  A = np.full((num_v, num_v, 2), float('inf'))
  A[:,:,0] = edges
  for k in range(num_v):
    print(k+1, 'of', num_v)
    for i in range(num_v):
      for j in range(num_v):
        A[i,j,(k+1)%2] = min(A[i,j,k%2],
                             A[i,k,k%2] + A[k,j,k%2])
  for i in range(len(edges)):
    if A[i, i, num_v%2] < 0:
      return None
  return np.amin(A[:,:, num_v%2])

def parse_args():
  parser = ArgumentParser()
  parser.add_argument('input_file')
  return parser.parse_args()
  
def main():
  args = parse_args()
  edges = get_edges(args.input_file)
  print('Shortest: ', get_shortest_path(edges))
  
if __name__ == '__main__':
  main()
