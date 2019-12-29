mod dijkstra {
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap};

    #[derive(Copy, Clone)]
    struct Edge {
        nid_to: u64,
        cost: u64,
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
        // Add edge to graph.
        pub fn add(&mut self, nid_from: u64, nid_to: u64, cost: u64) {
            let source = self.edges.get_mut(&nid_from);
            let edge = Edge { nid_to, cost };
            match source {
                Some(source_vec) => {
                    source_vec.push(edge);
                    None
                }
                None => self.edges.insert(nid_from, vec![edge]),
            };
        }

        fn edges(&self, source: u64) -> &Vec<Edge> {
            self.edges.get(&source).unwrap()
        }

        pub fn len(&self) -> usize {
            self.edges.len()
        }
    }

    struct PathLeg {
        edge: Edge,
        path_cost: u64,
    }

    impl Eq for PathLeg {}

    impl PartialEq for PathLeg {
        fn eq(&self, other: &Self) -> bool {
            self.path_cost.eq(&other.path_cost)
        }
    }

    // Reverse ordering so heap will be min-heap.
    impl Ord for PathLeg {
        fn cmp(&self, other: &Self) -> Ordering {
            other.path_cost.cmp(&self.path_cost)
        }
    }

    impl PartialOrd for PathLeg {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // Get a mapping of node ID to minimum cost to node.
    pub fn run(graph: &Graph, source: u64) -> HashMap<u64, u64> {
        let mut results = HashMap::new();
        // Initialize min heap with zero-cost path to source.
        let mut heap = BinaryHeap::new();
        heap.push(PathLeg {
            edge: Edge {
                nid_to: source,
                cost: 0,
            },
            path_cost: 0,
        });
        while results.len() != graph.len() {
            // Pop off heap until unvisited node found.
            while results.contains_key(&heap.peek().unwrap().edge.nid_to) {
                heap.pop();
            }
            let min_leg = heap.pop().unwrap();
            let dest = min_leg.edge.nid_to;
            let cost_to_dest = min_leg.path_cost;
            assert!(results.insert(dest, cost_to_dest).is_none());
            for &edge in graph.edges(dest) {
                let leg = PathLeg {
                    edge,
                    path_cost: cost_to_dest + edge.cost,
                };
                heap.push(leg);
            }
        }
        results
    }
} // dijkstra

use std::io::{self, Read};
// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
fn main() {
    let input = get_stdin().unwrap();
    let mut graph = dijkstra::Graph::new();

    for l in input.lines() {
        let mut opt_from: Option<u64> = None;
        for t in l.split_whitespace() {
            if opt_from.is_some() {
                let mut opt_to: Option<u64> = None;
                for tt in t.split(',') {
                    if opt_to.is_none() {
                        opt_to = Some(tt.parse::<u64>().unwrap());
                    } else {
                        let cost = tt.parse::<u64>().unwrap();
                        graph.add(opt_from.unwrap(), opt_to.unwrap(), cost);
                    }
                }
            } else {
                opt_from = Some(t.parse::<u64>().unwrap());
                assert!(opt_from.is_some());
            }
        }
    }

    let results = dijkstra::run(&graph, 1);
    println!("Cost to 7 is {}", results.get(&7).unwrap());
    println!("Cost to 37 is {}", results.get(&37).unwrap());
    println!("Cost to 59 is {}", results.get(&59).unwrap());
    println!("Cost to 82 is {}", results.get(&82).unwrap());
    println!("Cost to 99 is {}", results.get(&99).unwrap());
    println!("Cost to 115 is {}", results.get(&115).unwrap());
    println!("Cost to 133 is {}", results.get(&133).unwrap());
    println!("Cost to 165 is {}", results.get(&165).unwrap());
    println!("Cost to 188 is {}", results.get(&188).unwrap());
    println!("Cost to 197 is {}", results.get(&197).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut g = dijkstra::Graph::new();
        g.add(1, 2, 1.0);
        g.add(1, 3, 2.0);
        g.add(2, 3, 3.0);
        //assert_eq!(g.nodes.len(), 3);
        //assert_eq!(g.edges.len(), 3);
    }
}
