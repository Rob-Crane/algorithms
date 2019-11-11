// Reads a space separated list of integers from stdin and prints the number
// of sorting inversions.

use std::io::{self, Read};

mod inversions {
    pub fn count(numbers : &[i64]) -> usize {
        if numbers.len() == 1 {
            return 0;
        }
        solve_subproblem(numbers).1
    }
    fn solve_subproblem(subarr: &[i64]) -> (Vec<i64>, usize) {
        if subarr.len() == 1 {
            return (vec![subarr[0]], 0);
        }
        let m_ind = subarr.len() / 2;
        let (l_sorted, l_inversions) = solve_subproblem(&subarr[..m_ind]);
        let (r_sorted, r_inversions) = solve_subproblem(&subarr[m_ind..]);
        let (merged, split_invs) = merge(&l_sorted, &r_sorted);

        (merged, l_inversions + r_inversions + split_invs)
    }
    fn merge(left: &[i64], right: &[i64]) -> (Vec<i64>, usize) {
        let merged_sz = left.len() + right.len();
        let mut merged = Vec::with_capacity(merged_sz);
        let (mut l_i, mut r_i, mut invs) = (0, 0, 0);
        while l_i + r_i < merged_sz {
            if l_i < left.len() && (r_i == right.len() || left[l_i] < right[r_i]) {
                merged.push(left[l_i]);
                l_i += 1;
            } else {
                merged.push(right[r_i]);
                r_i += 1;
                invs+=left.len() - l_i;
            }
        }
        (merged, invs)
    }
}

// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!("Number of inversions: {}", inversions::count(&numbers));
}
