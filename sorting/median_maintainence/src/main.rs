use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

pub struct MedianTracker {
    lower: BinaryHeap<i64>,
    upper: BinaryHeap<Reverse<i64>>,
}
impl MedianTracker {
    pub fn new() -> MedianTracker {
        MedianTracker {
            lower: BinaryHeap::new(),
            upper: BinaryHeap::new(),
        }
    }
    pub fn median(&mut self) -> i64 {
        *self.lower.peek().unwrap()
    }
    pub fn insert(&mut self, x: i64) {
        if self.lower.len() == 0 || x < self.median() {
            // Insert into max heap.
            self.lower.push(x);
        } else {
            // Insert into min heap.
            self.upper.push(Reverse(x));
        }
        // Balance so the median is always the top of max heap.
        let even_elements = (self.lower.len() + self.upper.len()) % 2 == 0;
        if even_elements && self.lower.len() > self.upper.len() {
            let lower_top = self.lower.pop().unwrap();
            self.upper.push(Reverse(lower_top));
        } else if !even_elements && self.lower.len() < self.upper.len() {
            let upper_top = self.upper.pop().unwrap();
            self.lower.push(upper_top.0);
        }
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

    let mut tracker = MedianTracker::new();
    let mut sum = 0;

    for l in input.lines() {
        let i = l.parse::<i64>().unwrap();
        tracker.insert(i);
        sum += tracker.median();
    }
    println!("Sum of medians: {}", sum);
}
