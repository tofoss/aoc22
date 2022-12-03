use std::{fs, collections::HashSet};

const DAY: &str = "03";


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
    let result: usize = input.trim().split("\n")
        .map(find_priority)
        .sum();

    println!("Part one answer: {result}")
}


fn part_two(input: &String) {
    let result: usize = input.trim().split("\n")
        .array_chunks::<3>()
        .map(find_badge_priority)
        .sum();

    println!("Part two answer: {result}")
}


fn find_badge_priority(sacks: [&str; 3]) -> usize {
    return to_set(sacks[0])
        .intersection(&to_set(sacks[1]))
        .map(|i| i.to_owned())
        .collect::<HashSet<&u8>>()
        .intersection(&to_set(sacks[2]))
        .into_iter()
        .map(to_priority)
        .sum::<u8>() as usize;
}

fn find_priority(sack: &str) -> usize {
    let (c1, c2) = sack.split_at(sack.len() / 2);

    return to_set(c1)
        .intersection(&to_set(c2))
        .into_iter()
        .map(to_priority)
        .sum::<u8>() as usize;
}

fn to_priority(b: &&u8) -> u8 {
    if *b > &('Z' as u8) {
        return *b - ('a' as u8) + 1 
    } else {
        return *b - ('A' as u8) + 1 + 26
    }
}

fn to_set(c: &str) -> HashSet<&u8> {
    let mut set = HashSet::new();

    c.as_bytes().into_iter().for_each(|b| {set.insert(b);});

    return set
}
