use std::fs;

const DAY: &str = "13";

#[derive(Debug)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

#[derive(PartialEq, Eq)]
enum State {
    Unknown = 0,
    Fail = 1,
    Pass = -1,
}

fn right_order(left: &Packet, right: &Packet) -> State {
    match (left, right) {
        (Packet::Num(l), Packet::Num(r))   => cmp_num(l, r),
        (Packet::List(l), Packet::List(r)) => cmp_list(l, r),
        (Packet::Num(l), Packet::List(r))  => cmp_list(&vec![Packet::Num(*l)], r),
        (Packet::List(l), Packet::Num(r))  => cmp_list(l, &vec![Packet::Num(*r)]),
    }
}

fn cmp_num(l: &usize, r: &usize) -> State {
    if l == r {
        State::Unknown
    } else if l < r {
        State::Pass
    } else {
        State::Fail
    }
}

fn cmp_list(l: &Vec<Packet>, r: &Vec<Packet>) -> State {
    for (li, ri) in l.iter().zip(r) {
        let state = right_order(li, ri);
        if state != State::Unknown {
            return state;
        }
    }
    if l.len() < r.len() {
        return State::Pass;
    } if l.len() == r.len() {
        return State::Unknown;
    } else {
        return State::Fail;
    }
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
    let packets: Vec<Packet> = input
        .trim()
        .lines()
        .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
        .collect();

    let mut sum = 0;
    for i in (0..packets.len()).step_by(2) {
        if right_order(&packets[i], &packets[i+1]) ==  State::Pass {
            sum += i / 2 + 1;
        }
    }
        
    println!("Dec {DAY}:");
    println!("Part one sum: {}", sum);
    println!();
}

