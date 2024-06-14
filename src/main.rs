use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut count: HashMap<char, usize> = HashMap::new();
    let stdin = io::stdin();
    let mut lines = 0;
    let mut charinstances = 0;
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            lines += 1;
            for c in line.chars() {
                count.entry(c).and_modify(|n| *n += 1).or_insert(1);
                charinstances += 1;
            }
        }
    }
    eprintln!(
        "Found {} character types, {} character tokens in {} lines",
        count.len(),
        charinstances,
        lines,
    );
    let mut sorted: Vec<(&char, &usize)> = count.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (c, freq) in sorted {
        if *c == '\n' {
            println!("\\n\t{}", *freq);
        } else {
            println!("{}\t{}", *c, *freq);
        }
    }
}
