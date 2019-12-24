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
    let mut edge_map = scc::edge::EdgeMap::new();
    for line in input.lines() {
        let edge_nids: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        assert_eq!(edge_nids.len(), 2);
        edge_map.add(edge_nids[0], edge_nids[1]);
    }
    println!("Done reading input.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_add() {
        let mut edge_map = scc::edge::EdgeMap::new();
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
}
