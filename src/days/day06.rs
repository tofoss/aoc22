use std::{fs, collections::HashSet};

const DAY: &str = "06";

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();
        
    println!("Dec {DAY}:");
    part_one(&input);
    part_two(&input);
    println!();
}

fn part_one(input: &String) {
    let mut result = 0;
    let bs = input.as_bytes();
    for (i, w) in bs.windows(4).enumerate() {
        if w.iter().collect::<HashSet<&u8>>().len() == 4 {
            result = i + 4;
            break;
        }
    }

    println!("Part one answer: {result}")
}


fn part_two(input: &String) {
    let mut result = 0;
    let bs = input.as_bytes();
    for (i, w) in bs.windows(14).enumerate() {
        if w.iter().collect::<HashSet<&u8>>().len() == 14 {
            result = i + 14;
            break;
        }
    }

    println!("Part two answer: {result}")
}
