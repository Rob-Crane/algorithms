mod min_cut {
    use std::collections::{HashMap, HashSet};
    extern crate rand;
    use rand::{rngs, thread_rng, Rng};
    #[derive(Debug, Clone)]
    pub struct Edge {
        nid1: u64,
        nid2: u64,
    }
    #[derive(Debug, Clone)]
    pub struct Node {
        eids: HashSet<u64>,
    }
    impl Node {
        pub fn new() -> Node {
            return Node {
                eids: HashSet::new(),
            };
        }
        pub fn add(&mut self, eid: u64) {
            assert!(self.eids.insert(eid));
        }
        pub fn remove(&mut self, eid: u64) {
            assert!(self.eids.remove(&eid));
        }
        pub fn num_edges(&self) -> usize {
            self.eids.len()
        }
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        pub nodes: HashMap<u64, Node>,
        pub edges: HashMap<u64, Edge>,
        next_eid: u64,
    }
    impl Graph {
        pub fn new() -> Self {
            return Graph {
                nodes: HashMap::new(),
                edges: HashMap::new(),
                next_eid: 0,
            };
        }
        // Add an edge to the Graph.  Create Nodes if
        // they don't exist.
        pub fn add(&mut self, nid1: u64, nid2: u64) {
            assert_ne!(nid1, nid2);
            self.add_edge(nid1, nid2);
            self.maybe_add_node(nid1);
            self.maybe_add_node(nid2);
            self.next_eid += 1;
        }

        fn add_edge(&mut self, nid1: u64, nid2: u64) {
            let edge = Edge { nid1, nid2 };
            if self.edges.insert(self.next_eid, edge).is_some() {
                panic!("Already contains edge with ID {}", self.next_eid);
            }
        }

        fn maybe_add_node(&mut self, nid: u64) {
            if let Some(n) = self.nodes.get_mut(&nid) {
                n.add(self.next_eid);
            } else {
                let mut n = Node::new();
                n.add(self.next_eid);
                self.nodes.insert(nid, n);
            }
        }

        pub fn run_karger(&mut self) -> usize {
            let mut rng = rand::thread_rng();
            while self.nodes.len() > 2 {
                self.merge_random(&mut rng);
            }
            return self.edges.len();
        }

        // Merge random edge.
        fn merge_random(&mut self, rng: &mut rngs::ThreadRng) {
            let k = rng.gen_range(0, self.edges.len());
            let eid = *self.edges.keys().nth(k).unwrap();
            self.merge_edge(eid);
        }

        // Merge node 2 of edge into node 1.  All edges attached to
        // node are attached to node 1.  Edges between node 1 and 2
        // are removed.
        pub fn merge_edge(&mut self, eid: u64) {
            let e = &self.edges.get(&eid).unwrap();
            let nid1 = e.nid1;
            let nid2 = e.nid2;
            // Redirect second node's edges to first.
            let n2 = &self.nodes.get(&nid2).unwrap();
            // Track edges to add or remove from Node 1 after Node 2's edges
            // have been redirected or removed.
            let mut eids_to_add = Vec::<u64>::with_capacity(n2.eids.len());
            let mut eids_to_del = Vec::<u64>::with_capacity(n2.eids.len());
            for n2_eid in n2.eids.iter() {
                if Graph::redirect_edge_or_drop(&mut self.edges, *n2_eid, nid2, nid1) {
                    eids_to_add.push(*n2_eid);
                } else {
                    eids_to_del.push(*n2_eid);
                }
            }
            let n = self.nodes.get_mut(&nid1).unwrap();
            for e in eids_to_add {
                n.add(e);
            }
            for e in eids_to_del {
                n.remove(e);
            }
            // Delete the unmerged node.  Not strictly necessary for algorithm
            // but simplifies unit testing.
            self.nodes.remove(&nid2);
        }

        // Redirects an edge's endpoints or drops it from edge map
        // if redirection creates self-loop.
        fn redirect_edge_or_drop(
            edges: &mut HashMap<u64, Edge>,
            eid: u64,
            nid_old: u64,
            nid_new: u64,
        ) -> bool {
            let e = edges.get_mut(&eid).unwrap();
            let (to_update, no_update);
            if e.nid1 == nid_old {
                to_update = &mut e.nid1;
                no_update = &e.nid2;
            } else {
                to_update = &mut e.nid2;
                no_update = &e.nid1;
            }
            if *no_update == nid_new {
                // Drop Edge, merging would create self edge.
                edges.remove(&eid);
                return false;
            } else {
                // Redirect Edge.
                *to_update = nid_new;
                return true;
            }
        }
    }
}
use std::cmp;
use std::io::{self, Read};
// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let mut g_buffer = min_cut::Graph::new();
    for l in input.lines() {
        let mut opt_from = None;
        for t in l.split_whitespace() {
            if let Some(from) = opt_from {
                let to = t.parse::<u64>().unwrap();
                if to > from {
                    g_buffer.add(from, to);
                }
            } else {
                opt_from = Some(t.parse::<u64>().unwrap());
            }
        }
    }
    println!(
        "Created graph with {} nodes and {} edges",
        g_buffer.nodes.len(),
        g_buffer.edges.len()
    );
    let num_trials = 100;
    let mut min_so_far = g_buffer.edges.len();
    for i in 1..num_trials+1 {
        let mut g = g_buffer.clone();
        min_so_far = cmp::min(min_so_far, g.run_karger());
        println!("Completed trial {} of {}", i, num_trials);
    }
    println!("Smallest cut found has {} edges.", min_so_far);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut g = min_cut::Graph::new();
        g.add(1, 2);
        g.add(1, 3);
        g.add(2, 3);
        assert_eq!(g.nodes.len(), 3);
        assert_eq!(g.edges.len(), 3);
    }
    #[test]
    fn test_merge_to_one_edge() {
        let mut g = min_cut::Graph::new();
        g.add(1, 2);
        g.add(2, 3);
        g.merge_edge(1);
        for n in g.nodes.values() {
            assert_eq!(n.num_edges(), 1);
        }
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.edges.len(), 1);
    }

    #[test]
    fn test_merge_to_two_edge() {
        let mut g = min_cut::Graph::new();
        g.add(1, 2);
        g.add(2, 3);
        g.add(1, 3);
        g.merge_edge(2);
        for n in g.nodes.values() {
            assert_eq!(n.num_edges(), 2);
        }
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.edges.len(), 2);
    }

}
