mod scc;

use std::io::{self, Read};

// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let mut edge_map = scc::EdgeMap::new();
    for line in input.lines() {
        let edge_nids: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        assert_eq!(edge_nids.len(), 2);
        edge_map.add(edge_nids[0], edge_nids[1]);
    }
    println!("Done reading input.");
    let kfp = scc::KosarajuFirstPass::new(&edge_map);
    println!("Completed first pass.");
    let ksp = scc::KosarajuSecondPass::new(&edge_map, kfp.second_pass_ordering());
    let rev_counts = ksp.reverse_sorted_counts();
    println!("Top 5: {:?}", &rev_counts[..5]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_add() {
        let mut edge_map = scc::EdgeMap::new();
        edge_map.add(1, 2);
        edge_map.add(3, 2);
        edge_map.add(3, 4);
        edge_map.add(4, 2);

        let to_2: HashSet<&u64> = edge_map.nids_to(2).unwrap().collect();
        let to_2_exp = vec![&1, &3, &4].into_iter().collect();
        assert_eq!(to_2, to_2_exp);

        let from_3: HashSet<&u64> = edge_map.nids_from(3).unwrap().collect();
        let from_3_exp = vec![&2, &4].into_iter().collect();
        assert_eq!(from_3, from_3_exp);
    }

    #[test]
    fn test_first_pass() {
        let mut edge_map = scc::EdgeMap::new();
        edge_map.add(1, 2);
        edge_map.add(2, 3);
        let kfp = scc::KosarajuFirstPass::new(&edge_map);
        let ordering : Vec<&u64> = kfp.second_pass_ordering().collect(); 
        assert_eq!(ordering, vec![&3,&2,&1]);
    }

    #[test]
    fn test_second_pass() {
        let mut edge_map = scc::EdgeMap::new();
        edge_map.add(1, 4);
        edge_map.add(2, 8);
        edge_map.add(3, 6);
        edge_map.add(4, 7);
        edge_map.add(5, 2);
        edge_map.add(6, 9);
        edge_map.add(7, 1);
        edge_map.add(8, 5);
        edge_map.add(8, 6);
        edge_map.add(9, 7);
        edge_map.add(9, 3);
        let kfp = scc::KosarajuFirstPass::new(&edge_map);
        let ksp = scc::KosarajuSecondPass::new(&edge_map, kfp.second_pass_ordering());
        let rev_counts = ksp.reverse_sorted_counts();
        assert_eq!(rev_counts[0], 3);
        assert_eq!(rev_counts[1], 3);
        assert_eq!(rev_counts[2], 3);
        assert_eq!(rev_counts[3], 0);
        assert_eq!(rev_counts[4], 0);
        assert_eq!(rev_counts[5], 0);
    }
}
