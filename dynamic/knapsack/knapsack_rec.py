from argparse import ArgumentParser
import sys
sys.setrecursionlimit(10000)
class Item:
  def __init__(self, value, weight):
    self.value = value
    self.weight = weight

class Knapsack:
  def __init__(self, items):
    self.items = items
    self.memo = [{} for _ in range(len(items))]
  
  def value(self, i, x):
    
    if i < 0:
        return 0
    if x in self.memo[i]:
      return self.memo[i][x]
    
    if x < self.items[i].weight:
      val = self.value(i-1, x)

    else:
      item = self.items[i]
      val = max(self.value(i-1, x),
                self.value(i-1, x-item.weight) + item.value)
    self.memo[i][x] = val
    return val


def knapsack_value(items, capacity):
  return Knapsack(items).value(len(items)-1, capacity)


def read_input(in_file):
    capacity, num_items = None, None
    items = []
    with open(in_file) as f:
        for l in f:
            n1, n2 = map(int, l.split())
            if not capacity:
                capacity, num_items = n1, n2
                continue
            items.append(Item(n1, n2))
    assert(len(items) == num_items)
    return items, capacity


def parse_args():
    parser = ArgumentParser()
    parser.add_argument('--in_file', required=True)
    return parser.parse_args()


def main():
    args = parse_args()
    items, capacity = read_input(args.in_file)
    max_value = knapsack_value(items, capacity)
    print('Max knapsack value: ', max_value)

if __name__ == '__main__':
   main()
