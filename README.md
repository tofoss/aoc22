# AOC 2022

First AOC, first time using Rust.

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

fn part_one(input: &_) {
    let result: usize = 0;

    println!("Part one answer: {result}")
}


fn part_two(input: &_) {
    let result: usize = 0;

    println!("Part two answer: {result}")
}
```
