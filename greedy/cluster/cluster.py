from argparse import ArgumentParser
import heapq

class CostTable:

    def __init__(self):
        self.table = {}

    @staticmethod
    def from_file(fname):
        table = CostTable()
        with open(fname) as f:
            num_costs = 0
            for l in f:
                if not num_costs:
                    num_costs = int(l)
                else:
                    [n0, n1, cost] = [int(f) for f in l.split()]
                    table._add_cost(n0, n1, cost)
                    table._add_cost(n1, n0, cost)
        return table

    def get(self, nid0, nid1):
        if nid0 in self.table and nid1 in self.table[nid0]:
            return self.table[nid0][nid1]
        raise RuntimeError('Unable to find cost.')

    def _add_cost(self, nid0, nid1, cost):
        if nid0 in self.table:
            assert nid1 not in self.table[nid0]
            self.table[nid0][nid1] = cost
        else:
            self.table[nid0] = {nid1: cost}

def get_max_dist(cost_table, num_clusters):

    # Initialize heap from cost_table.
    cost_heap = []
    for nid0 in cost_table.table:
        for nid1 in cost_table.table[nid0]:
            if nid0 < nid1:
                cost_heap.append((cost_table.get(nid0, nid1), nid0, nid1))
    heapq.heapify(cost_heap)

    # Store clusters keyed by starting node.
    clusters = {nid : [nid] for nid in cost_table.table}
    nodes = {nid: nid for nid in cost_table.table}
    while len(clusters) > num_clusters:
        cost, nid0, nid1 = heapq.heappop(cost_heap)
        cluster0 = nodes[nid0]
        cluster1 = nodes[nid1]
        if cluster0 != cluster1:
            if len(clusters[cluster0]) < len(clusters[cluster1]):
                smaller = cluster0
                larger = cluster1
            else:
                smaller = cluster1
                larger = cluster0
            for nid in clusters[smaller]:
                nodes[nid] = larger
            clusters[larger].extend(clusters[smaller])
            del clusters[smaller]

    min_cost = float('inf')
    for nid0 in nodes:
        for nid1 in nodes:
            if nodes[nid0] != nodes[nid1]:
                min_cost = min(min_cost, cost_table.get(nid0, nid1))
    return min_cost

            

def parse_args():
    parser = ArgumentParser(description='Compute max distance of greeding clustering.')
    parser.add_argument('--input_file', required=True)
    parser.add_argument('--num_clusters', required=True)
    return parser.parse_args()


def main():
    args = parse_args()
    cost_table = CostTable.from_file(args.input_file)
    max_dist = get_max_dist(cost_table, int(args.num_clusters))
    print(f'Max dist for {args.num_clusters} clusters is {max_dist}.')


if __name__ == '__main__':
    main()
