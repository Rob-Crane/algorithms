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

    struct ClusterTracker {
        nodes: HashMap<u64, u64>,
        clusters: HashMap<u64, Vec<u64>>,
    }

    impl ClusterTracker {
        fn add_node(&mut self, nid: u64) {
            self.nodes.insert(nid, nid);
            self.clusters.insert(nid, vec![nid]);
        }
    }

} // mod hamming
fn main() {
    for i in hamming::HammingVariations::new(1, 1) {
        println!("Variation: {}", i);
    }
}
