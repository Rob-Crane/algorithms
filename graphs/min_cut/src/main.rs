mod min_cut {
    use std::collections::{HashMap, HashSet};
    #[derive(Debug)]
    pub struct Edge {
        nid1: u64,
        nid2: u64,
    }
    #[derive(Debug)]
    pub struct Node {
        eids: HashSet<u64>,
    }

    impl Node {
        pub fn new() -> Node {
            return Node {eids : HashSet::new()};
        }
        pub fn add(&mut self, eid: u64) {
          assert!(self.eids.insert(eid));
        }
        pub fn remove(&mut self, eid: u64) {
          assert!(self.eids.remove(&eid));
        }
    }

    #[derive(Debug)]
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
        pub fn add(&mut self, nid1: u64, nid2: u64) {
            assert_ne!(nid1, nid2);
            self.add_edge(nid1, nid2);
            self.maybe_add_node(nid1);
            self.maybe_add_node(nid2);
            self.next_eid += 1;
        }

        fn add_edge(&mut self, nid1: u64, nid2: u64) {
            // Add Edge.
            let edge = Edge { nid1, nid2 };
            if self.edges.insert(self.next_eid, edge).is_some() {
                panic!("Already contains edge with ID {}", self.next_eid);
            }
        }

        fn maybe_add_node(&mut self, nid: u64) {
            let opt_n = self.nodes.get_mut(&nid);
            if let Some(n) = opt_n {
                n.add(self.next_eid);
            } else {
                let mut n = Node::new();
                n.add(self.next_eid);
                self.nodes.insert(nid, n);
            }
        }

        // Merge node 2 of edge into node 1.
        pub fn merge_edge(&mut self, eid: u64) {
            let e = &self.edges.get(&eid).unwrap();
            let nid1 = e.nid1;
            let nid2 = e.nid2;
            // Redirect second node's edges to first.
            let n2 = &self.nodes.get(&nid2).unwrap();
            let mut eids_to_add = Vec::<u64>::new();
            let mut eids_to_del = Vec::<u64>::new();
            for n2_eid in n2.eids.iter() {
                if Graph::redirect_edge_or_drop(&mut self.edges, *n2_eid, nid2, nid1) {
                    // Add edge ID to node.
                    eids_to_add.push(*n2_eid);
                } else {
                    eids_to_del.push(*n2_eid);
                }
            }
            let n = self.nodes
                .get_mut(&nid1)
                .unwrap();
            for e in eids_to_add {
                n.add(e);
            }
            for e in eids_to_del {
                n.remove(e);
            }
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
fn main() {
    println!("Hello, world!");
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
    fn test_merge() {
        let mut g = min_cut::Graph::new();
        g.add(1, 2);
        g.add(2, 3);
        g.add(1, 3);
        println!("Edges: {:?}", g.edges);
        g.merge_edge(2);
        println!("Edges: {:?}", g.edges);
    }

}
