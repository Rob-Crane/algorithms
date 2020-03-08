from argparse import ArgumentParser


def knapsack_max(items, capacity):
    ledger = {}
    # Stack entries are subproblem dependencies of 
    # entries lower on the stack.
    subproblem_stack = [(len(items)-1, capacity)]
    num_iter = 0
    while subproblem_stack:
        i, x = subproblem_stack.pop()
        assert i >= 0 and x >= 0
        no_take_sol, take_sol = None, None
        if i == 1:
            no_take_sol = 0
            take_sol = 0
        else:
            no_take_sol = ledger.get((i-1, x))
            if x < items[i].weight:
                take_sol = no_take_sol
            else:
                take_sol = ledger.get((i-1, x-items[i].weight))

        if no_take_sol is None or take_sol is None:
            subproblem_stack.append((i, x))
            if no_take_sol is None:
                subproblem_stack.append((i-1, x))
            if take_sol is None and x >= items[i].weight:
                subproblem_stack.append((i-1, x-items[i].weight))
        else:
            if x < items[i].weight:
                ledger[(i,x)] = no_take_sol
            else:
                ledger[(i,x)] = max(no_take_sol, take_sol + items[i].value)
        num_iter+=1
        # if not num_iter%100000:
            # print('Iteration: {} Ledger Size: {} Queue Size: {}'.format(num_iter, len(ledger), len(subproblem_stack)))
    return ledger[(len(items)-1, capacity)]


class Item:
    def __init__(self, value, weight):
        self.value = value
        self.weight = weight


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
    max_value = knapsack_max(items, capacity)
    print('Max knapsack value: ', max_value)

if __name__ == '__main__':
   main()
