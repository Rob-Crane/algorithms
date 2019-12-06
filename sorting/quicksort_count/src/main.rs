use std::io;
use std::io::Read;

mod qscount {

    #[derive(PartialEq, Clone, Copy)]
    pub enum Pivot {
        First,
        Last,
        MedianOfThree,
    }

    pub fn count(numbers: &mut [i64], pivot: Pivot) -> usize {
        if numbers.len() <= 1 {
            return 0;
        }

        // Move pivot to first element of array.
        if pivot == Pivot::Last {
            numbers.swap(0, numbers.len() - 1);
        } else if pivot == Pivot::MedianOfThree {
            // Pivot is median of first, middle, last element.
            let mid_i = (numbers.len() + 1) / 2 - 1;
            let last_i = numbers.len() - 1;
            let mut candidates = [
                (0, numbers[0]),
                (mid_i, numbers[mid_i]),
                (last_i, numbers[last_i]),
            ];
            candidates.sort_by(|a, b| a.1.cmp(&b.1));
            numbers.swap(0, candidates[1].0);
        }

        // Partition operation.
        let mut i = 1; // Index of first number greater than pivot.
        for j in 1..numbers.len() {
            if numbers[j] < numbers[0] {
                numbers.swap(i, j);
                i += 1;
            }
        }
        numbers.swap(0, i - 1); // Replace pivot.
        return numbers.len() - 1
            + count(&mut numbers[0..i - 1], pivot)
            + count(&mut numbers[i..], pivot);
    }
}

// Read stdin to a string.
fn get_stdin() -> std::io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let mut numbers: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!(
        "Pivot on first: {} comparisons.",
        qscount::count(&mut numbers.clone(), qscount::Pivot::First)
    );
    println!(
        "Pivot on last: {} comparisons.",
        qscount::count(&mut numbers.clone(), qscount::Pivot::Last)
    );
    println!(
        "Pivot on 'median of three': {} comparisons.",
        qscount::count(&mut numbers, qscount::Pivot::MedianOfThree)
    );
}
