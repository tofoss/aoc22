use std::{fs, str::FromStr, string::ParseError};

const DAY: &str = "04";

#[derive(Debug)]
struct Pair {
    a: [u32; 2],
    b: [u32; 2],
}

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn _into_sec(ss: &str) -> [u32; 2] {
            return ss
                .split("-")
                .into_iter()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        }

        let (a, b) = s.trim().split_once(",").unwrap();

        Ok(Pair {
            a: _into_sec(a),
            b: _into_sec(b),
        })
    }
}

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
    let result: usize = input
        .trim()
        .split("\n")
        .map(|p| p.parse::<Pair>().unwrap())
        .map(|p| if is_contained(&p) { 1 } else { 0 })
        .sum();

    println!("Part one answer: {result}")
}

fn part_two(input: &String) {
    let result: usize = input
        .trim()
        .split("\n")
        .map(|p| p.parse::<Pair>().unwrap())
        .map(|p| if is_overlap(&p) { 0 } else { 1 })
        .sum();

    println!("Part two answer: {result}")
}

fn is_overlap(pair: &Pair) -> bool {
    return pair.a[1] < pair.b[0] || pair.b[1] < pair.a[0];
}

fn is_contained(pair: &Pair) -> bool {
    return cmp(pair.a, pair.b) || cmp(pair.b, pair.a);
}

fn cmp(a: [u32; 2], b: [u32; 2]) -> bool {
    return a[0] <= b[0] && a[1] >= b[1];
}
