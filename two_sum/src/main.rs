use std::collections::HashSet;
use std::io::{self, Read};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// Read stdin to a string.
fn get_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let input = get_stdin().unwrap();
    let mut numbers = HashSet::new();
    for l in input.lines() {
        let i = l.parse::<i64>().unwrap();
        numbers.insert(i);
    }
    println!("Completed table creation.");
    let numbers_lock = Arc::new(RwLock::new(numbers));
    let count_lock = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for t in -10000..10001 {
        let count_lock = count_lock.clone();
        let numbers_lock = numbers_lock.clone();
        handles.push(thread::spawn(move || {
            let numbers = numbers_lock.read().unwrap();
            for &x in numbers.iter() {
                let y = t-x;
                if x != y && numbers.contains(&y) {
                    let mut count = count_lock.lock().unwrap();
                    *count += 1;
                    break;
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Count: {}", *count_lock.lock().unwrap());
}
