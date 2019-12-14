mod min_cut {
    use std::collections::HashMap;
    struct Edge {
        nid1: u64,
        nid2: u64,
    }
    struct Node {
        eids: Vec<u64>,
    }

    struct Graph {
        nodes: HashMap<u64, Node>,
        edges: HashMap<u64, Edge>,
        next_eid: u64,
    }
    impl Graph {
        pub fn add(&mut self, nid1: u64, nid2: u64) {
            self.add_edge(nid1, nid2);
            self.update_node(nid1);
            self.update_node(nid2);
            self.next_eid += 1;
        }

        fn add_edge(&mut self, nid1: u64, nid2: u64) {
            // Add Edge.
            let edge = Edge { nid1, nid2 };
            if let Some(e) = self.edges.insert(self.next_eid, edge) {
                panic!("Already contains edge with ID {}", self.next_eid);
            }
        }

        fn update_node(&mut self, nid: u64) {
            let new_node = Node { eids: vec![self.next_eid] };
            if let Some(n) = self.nodes.insert(nid, new_node) {
                n.eids.push(self.next_eid);
            }
        }

        // Draw a random edge.
        //fn random_eid(&self) -> u64 {
        //}

        // Merge the nodes of this edge.
        fn merge_node(&mut self, eid: u64) {
            let e = &self.edges.get(eid).unwrap();
            // Redirect second node's edges to first.
            let n2 = &mut self.nodes.get(e.nid2).unwrap();
            for n2_eid in n2.eids {
                self.redirect_edge_or_drop(n2_eid, e.nid2, e.nid1);
            }
        }

        fn redirect_edge_or_drop(&mut self, eid: u64, nid_old: u64, nid_new: u64) {
            let e = &mut self.edges.get(eid).unwrap();
            let to_update, no_update;
            if e.nid1 == nid_old {
                to_update = &mut e.nid1;
                no_update = &e.nid2;
            } else {
                to_update = &mut e.nid2;
                no_update = &e.nid1;
            }
            if no_update == nid_new {
                // Drop Edge, merging would create self edge.
                self.edges.drop(eid);
            } else {
                // Redirect Edge.
                to_update = nid_new;
                // Add eid to new node.
                self.nodes.get(nid_new).unwrap().eids.push(eid);
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
}
