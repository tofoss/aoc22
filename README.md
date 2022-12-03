# AOC

First time using Rust please don't judge.

## Day template
```rust
use std::fs;

const DAY: &str = "00";

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(test).unwrap();
        
    println!("Dec {DAY}:");
    part_one(&input);
    part_two(&input);
    println!();
}

fn part_one(input: &String) {
    let result: usize = input.trim().split("\n").map(|_| 0).sum();

    println!("Part one answer: {result}")
}


fn part_two(input: &String) {
    let result: usize = input.trim().split("\n").map(|_| 0).sum();

    println!("Part two answer: {result}")
}
```
