pub mod edge {
    use std::collections::{HashMap, HashSet};

    // Describe the connections to a single node.
    // nids_to: Neighbors pointing to the node.
    // nids_from: Neighbors the node points to.
    struct EdgeList {
        nids_to: HashSet<u64>,
        nids_from: HashSet<u64>,
    }

    impl EdgeList {
        pub fn new() -> EdgeList {
            EdgeList {
                nids_to: HashSet::<u64>::new(),
                nids_from: HashSet::<u64>::new(),
            }
        }
        pub fn add_to(&mut self, nid_to: u64) {
            assert!(self.nids_to.insert(nid_to));
        }
        pub fn add_from(&mut self, nid_from: u64) {
            assert!(self.nids_from.insert(nid_from));
        }
        pub fn nids_to(&self) -> impl Iterator<Item = &u64> {
            self.nids_to.iter()
        }
        pub fn nids_from(&self) -> impl Iterator<Item = &u64> {
            self.nids_from.iter()
        }
    }

    pub struct EdgeMap {
        edges: HashMap<u64, EdgeList>,
    }

    impl EdgeMap {
        pub fn new() -> EdgeMap {
            EdgeMap {
                edges: HashMap::<u64, EdgeList>::new(),
            }
        }
        pub fn add(&mut self, nid_from: u64, nid_to: u64) {
            (*self.edges.entry(nid_from).or_insert(EdgeList::new())).add_from(nid_to);
            (*self.edges.entry(nid_to).or_insert(EdgeList::new())).add_to(nid_from);
        }
        // For a node, get all the nodes it points directly to.
        pub fn nids_to(&self, nid: u64) -> Option<impl Iterator<Item = &u64>> {
            if let Some(edge_list) = self.edges.get(&nid) {
                return Some(edge_list.nids_to());
            }
            None
        }

        // For a node, get all the nodes which point directly to it.
        pub fn nids_from(&self, nid: u64) -> Option<impl Iterator<Item = &u64>> {
            if let Some(edge_list) = self.edges.get(&nid) {
                return Some(edge_list.nids_from());
            }
            None
        }
    }
} // edge
