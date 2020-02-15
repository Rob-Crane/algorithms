mod prims {

    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap, HashSet};

    #[derive(Copy, Clone)]
    struct Edge {
        nid_to: u64,
        cost: i64,
    }

    pub struct Graph {
        edges: HashMap<u64, Vec<Edge>>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                edges: HashMap::<u64, Vec<Edge>>::new(),
            }
        }
        fn add_edge(&mut self, nid_from: u64, nid_to: u64, cost: i64) {
            let edge = Edge { nid_to, cost };
            let node = self.edges.get_mut(&nid_from);
            match node {
                Some(edges) => {
                    edges.push(edge);
                    None
                }
                None => self.edges.insert(nid_from, vec![edge]),
            };
        }
        // Add edge to graph.
        pub fn add(&mut self, nid0: u64, nid1: u64, cost: i64) {
            self.add_edge(nid0, nid1, cost);
            self.add_edge(nid1, nid0, cost);
        }

        fn edges(&self, source: u64) -> &Vec<Edge> {
            self.edges.get(&source).unwrap()
        }

        // Number of nodes in graph.
        pub fn len(&self) -> usize {
            self.edges.len()
        }
    }

    impl Eq for Edge {}

    impl PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.cost.eq(&other.cost)
        }
    }

    // Reverse ordering so heap will be min-heap.
    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn mst_cost(graph: &Graph, source: u64) -> i64 {
        let mut edge_heap: BinaryHeap<Edge> = BinaryHeap::new();
        let mut seen = HashSet::new();

        // Initialize edge heap with 0-cost edge to start point.
        edge_heap.push(Edge {
            nid_to: source,
            cost: 0,
        });

        let mut mst_cost = 0;
        while seen.len() != graph.len() {
            // Pop off heap until unvisited node found.
            while seen.contains(&edge_heap.peek().unwrap().nid_to) {
                edge_heap.pop();
            }
            let min_edge = edge_heap.pop().unwrap();
            mst_cost += min_edge.cost;
            seen.insert(min_edge.nid_to);
            for &edge in graph.edges(min_edge.nid_to) {
                edge_heap.push(edge);
            }
        }
        return mst_cost;
    }

} // mod prims

use std::io::{self, Read};
// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let mut graph = prims::Graph::new();
    let mut lines = input.lines();
    lines.next();

    for l in lines {
        let tokens: Vec<&str> = l.split_whitespace().collect();
        let nid0 = tokens[0].parse::<u64>().unwrap();
        let nid1 = tokens[1].parse::<u64>().unwrap();
        let cost = tokens[2].parse::<i64>().unwrap();
        graph.add(nid0, nid1, cost);
    }

    let results = prims::mst_cost(&graph, 1);
    println!("MST cost is {}", results);
}
