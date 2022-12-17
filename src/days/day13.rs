use std::{fs, cmp::Ordering};

const DAY: &str = "13";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r))   => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => cmp_list(l, r),
            (Packet::Num(l), Packet::List(r))  => cmp_list(&vec![Packet::Num(*l)], r),
            (Packet::List(l), Packet::Num(r))  => cmp_list(l, &vec![Packet::Num(*r)]),
        }
    }
}

fn cmp_list(l: &Vec<Packet>, r: &Vec<Packet>) -> Ordering {
    return l.cmp(r);
}


fn parse(s: &str) -> Packet {
    if &s[0..1] == "[" {
        let mut stack: usize = 0;
        let mut packet = String::new();
        let mut list = Vec::new();

        for c in s[1..s.len() - 1].chars() {
            if c == '[' {
                stack += 1;
            } else if c == ']' {
                stack -= 1;
            } 

            if c == ',' && stack == 0 {
                list.push(parse(packet.as_str()));
                packet = String::new();
            } else {
                packet.push(c)
            }
        }

        if !packet.is_empty() {
            list.push(parse(packet.as_str()));
        }

        Packet::List(list)
    } else {
        Packet::Num(s.parse().unwrap())
    }
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();
    let mut packets: Vec<Packet> = input
        .trim()
        .lines()
        .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
        .collect();

    let mut sum = 0;
    for i in (0..packets.len()).step_by(2) {
        if &packets[i] < &packets[i+1]{
            sum += i / 2 + 1;
        }
    }

    let divider1 = Packet::List(vec![Packet::Num(2)]);
    let divider2 = Packet::List(vec![Packet::Num(6)]);

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    let mut first_key = 0;
    let mut second_key = 0;

    for (i, p) in packets.iter().enumerate() {
        if p == &divider1 {
            first_key = i + 1;
        }
        if p == &divider2 {
            second_key = i + 1;
        }
    }
        
    println!("Dec {DAY}:");
    println!("Part one sum: {}", sum);
    println!("Part two sum: {}", first_key * second_key);
    println!();
}

