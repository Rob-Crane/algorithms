mod hamming {

    use std::collections::HashMap;

    pub struct HammingVariations {
        base: u32,
        positions: Vec<usize>,
        done: bool,
    }

    impl HammingVariations {
        pub fn new(base: u32, dist: usize) -> HammingVariations {
            assert!(dist <= 3);
            return HammingVariations {
                base,
                positions: (0..dist).collect(),
                done: false,
            };
        }
    }

    impl Iterator for HammingVariations {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.done {
                return None;
            }

            // Compute hamming variation from current positions.
            let mut ret = self.base;
            for shift in &self.positions {
                ret ^= 1u32 << shift;
            }

            // Increment positions.
            for i in 0..self.positions.len() {
                if self.positions[i] == 31 - self.positions.len() + 1 + i {
                    if i == 0 {
                        self.done = true;
                    } else {
                        self.positions[i] = self.positions[i - 1] + 1;
                    }
                } else if i == self.positions.len() - 1
                    || self.positions[i + 1] == 31 - self.positions.len() + 2 + i
                {
                    self.positions[i] += 1;
                }
            }
            Some(ret)
        }
    }

    pub struct ClusterTracker {
        nodes: HashMap<u32, u32>,
        clusters: HashMap<u32, Vec<u32>>,
    }

    impl ClusterTracker {
        pub fn new() -> ClusterTracker {
            ClusterTracker {
                nodes: HashMap::new(),
                clusters: HashMap::new(),
            }
        }

        pub fn num_clusters(&self) -> usize {
            self.clusters.len()
        }

        pub fn add_node(&mut self, nid: u32) {
            self.nodes.insert(nid, nid);
            self.clusters.insert(nid, vec![nid]);
        }

        pub fn merge(&mut self, from_nid: u32, to_nid: u32) {
            let to_cluster = self.nodes[&to_nid];
            let from_cluster = self.nodes[&from_nid];
            let (smaller, larger);
            if self.clusters[&to_cluster].len() < self.clusters[&from_cluster].len() {
                smaller = to_cluster;
                larger = from_cluster;
            } else {
                smaller = from_cluster;
                larger = to_cluster;
            }
            for nid in self.clusters.get_mut(&smaller).unwrap().iter_mut() {
                *self.nodes.get_mut(&nid).unwrap() = larger;
            }
            let smaller_clone = self.clusters[&smaller].clone();
            self.clusters.get_mut(&larger).unwrap().extend(smaller_clone);
            self.clusters.remove(&smaller);
        }

        pub fn cluster(&mut self, dist: usize) {
            let nids: Vec<u32> = self.nodes.keys().cloned().collect();
            let mut cnt = 0;
            for nid in nids.iter().cycle() {
                cnt +=1;
                for n_try in HammingVariations::new(*nid, dist) {
                    if self.nodes.contains_key(&n_try) && self.nodes[&nid] != self.nodes[&n_try] {
                        self.merge(*nid, n_try);
                        cnt = 0;
                    }
                }
                if cnt == nids.len() {
                    break;
                }
            }
        }
    }

} // mod hamming


use std::io::{self, Read};
// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let mut n_lines : Option<usize> = None;
    let mut tracker = hamming::ClusterTracker::new();
    for l in input.lines() {
        let mut tokens = l.split_whitespace();
        if n_lines.is_some() {
            let tvec : Vec<&str> = tokens.collect();
            tracker.add_node(u32::from_str_radix(&tvec.join(""), 2).unwrap());
        } else {
            n_lines = Some(tokens.next().unwrap().parse::<usize>().unwrap());
        }
    }
    println!("Input read complete.");
    tracker.cluster(1);
    tracker.cluster(2);
    println!("Have {} clusters.", tracker.num_clusters());
}
