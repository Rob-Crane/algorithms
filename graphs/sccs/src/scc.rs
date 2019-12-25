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

    pub fn nids(&self) -> impl Iterator<Item = &u64> {
        self.edges.keys()
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }
}

// Track the nodes of graph visisted during DFS.
struct VisitedMap {
    visited: HashMap<u64, bool>,
}

impl VisitedMap {
    pub fn new(edge_map: &EdgeMap) -> VisitedMap {
        VisitedMap {
            visited: edge_map.nids().map(|n| (*n, false)).collect(),
        }
    }

    pub fn mark_visited(&mut self, nid: u64) {
        let old = self.visited.insert(nid, true).unwrap();
        assert!(!old); // Assert node wasn't previously visited.
    }

    pub fn is_visited(&self, nid: u64) -> bool {
        *self.visited.get(&nid).unwrap()
    }
}

pub struct KosarajuFirstPass<'a> {
    edge_map: &'a EdgeMap,
    visited: VisitedMap,
    ordering: Vec<u64>,
}

impl<'a> KosarajuFirstPass<'a> {
    pub fn new(edge_map: &'a EdgeMap) -> KosarajuFirstPass {
        let mut kfp = KosarajuFirstPass {
            edge_map,
            visited: VisitedMap::new(edge_map),
            ordering: Vec::<u64>::with_capacity(edge_map.len()),
        };
        for &nid in kfp.edge_map.nids() {
            if !kfp.visited.is_visited(nid) {
                kfp.dfs(nid);
            }
        }
        kfp
    }
    pub fn second_pass_ordering(&self) -> impl Iterator<Item = &u64> {
        self.ordering.iter().rev()
    }
    fn dfs(&mut self, nid_start: u64) {

        let mut to_process = Vec::<Vec<u64>>::new();
        to_process.push(vec![nid_start]);
        self.visited.mark_visited(nid_start);
        while !to_process.is_empty() {
            let mut descendents = to_process.pop().unwrap();
            let nid = *descendents.last().unwrap();
            let mut first_child_ind : Option<usize> = None;
            for &n in self.edge_map.nids_to(nid).unwrap() {
                if !self.visited.is_visited(n) {
                    self.visited.mark_visited(n);
                    if first_child_ind.is_none() {
                        first_child_ind = Some(to_process.len());
                    }
                    to_process.push(vec![n]);
                }

            }
            if let Some(ind) = first_child_ind {
                descendents.push(to_process[ind].pop().unwrap());
                to_process[ind] = descendents;
            } else {
                while !descendents.is_empty() {
                    self.ordering.push(descendents.pop().unwrap());
                }

            }
        }
        //self.visited.mark_visited(nid_start);
        //for &n in self.edge_map.nids_to(nid_start).unwrap() {
            //if !self.visited.is_visited(n) {
                //self.dfs(n);
            //}
        //}
        //self.ordering.push(nid_start);
    }
}

struct ConnectedComponentCounts {
    counts: HashMap<u64, usize>,
}

impl ConnectedComponentCounts {
    pub fn new(edge_map: &EdgeMap) -> ConnectedComponentCounts {
        ConnectedComponentCounts {
            counts: edge_map.nids().map(|n| (*n, 0)).collect(),
        }
    }

    pub fn increment(&mut self, nid: u64) {
        let cnt = self.counts.get_mut(&nid).unwrap();
        *cnt += 1;
    }

    pub fn counts(&self) -> impl Iterator<Item = &usize> {
        self.counts.values()
    }
}

pub struct KosarajuSecondPass<'a> {
    edge_map: &'a EdgeMap,
    visited: VisitedMap,
    scc_counts: ConnectedComponentCounts
}

impl<'a, 'b> KosarajuSecondPass<'a>  {

    pub fn new<T: Iterator<Item = &'b u64>> (edge_map: &'a EdgeMap, ordering: T) -> KosarajuSecondPass {
        let mut ksp = KosarajuSecondPass{
            edge_map,
            visited: VisitedMap::new(edge_map),
            scc_counts: ConnectedComponentCounts::new(edge_map)
        };
        for &nid_source in ordering {
            if !ksp.visited.is_visited(nid_source) {
                ksp.dfs(nid_source, nid_source);
            }
        }
        ksp
    }

    pub fn reverse_sorted_counts(&self) -> Vec<usize> {
        let mut ret : Vec<usize> = self.scc_counts.counts().map(|x| *x).collect();
        ret.sort_unstable_by(|a, b| b.cmp(a));
        ret
    }

    fn dfs(&mut self, nid: u64, nid_source: u64) {
        self.visited.mark_visited(nid);
        self.scc_counts.increment(nid_source);
        for &n in self.edge_map.nids_from(nid).unwrap() {
            if !self.visited.is_visited(n) {
                self.dfs(n, nid_source);
            }
        }
    }
}
