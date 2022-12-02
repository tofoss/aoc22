use std::fs;

// Rock: A => X => 1
// Paper: B => Y => 2
// Scissor: C => Z => 3
// 1, 2 
// 2, 1
// 3, 3

pub fn solve() {
    let input = fs::read_to_string("input/day02-1.txt")
        .expect("file not found");
        
    println!("Dec 02:");
    part_one(&input);
    part_two(&input);
    println!();
}

fn part_one(input: &String) {
    let result: u32 = input.trim().split("\n").map(|game| { 
        let mut foo = game.split(" ").map(|s| map_rps(s));
        return rps(foo.next().unwrap(), foo.next().unwrap());
    }).sum();


    println!("Part one answer: {result}")
}


fn part_two(input: &String) {
    // X => loss, Y => draw, Z => win
    let result: u32 = input.trim().split("\n").map(|game| { 
        let foo = map_strat(game);
        return rps(foo[0], foo[1]);
    }).sum();


    println!("Part two answer: {result}")
}

fn map_strat(game: &str) -> Vec<u32>{
    let (other, strat) = game.split_once(" ").unwrap();
    if strat == "Y" {
        return vec![map_rps(other), map_rps(other)]
    }

    let mut res = Vec::new();
    res.push(map_rps(other));

    if strat == "Z" {
        match res[0] {
            1 => res.push(2),
            2 => res.push(3),
            3 => res.push(1),
            _ => res.push(0),
        }
    } else {
        match res[0] {
            1 => res.push(3),
            2 => res.push(1),
            3 => res.push(2),
            _ => res.push(0),
        }
    }
    return res;
}

fn rps(other: u32, us: u32) -> u32 {
    if other == us {
        return us + 3;
    }

    let res: u32;
    match us {
       1 => if other == 2  { res = us } else { res = us + 6 }
       2 => if other == 3  { res = us } else { res = us + 6 }
       3 => if other == 1  { res = us } else { res = us + 6 }
       _ => res = 0
    }
    return res;
}

fn map_rps(s: &str) -> u32 {
    match s.chars().next().unwrap() {
        'A' =>  1,
        'X' =>  1,
        'B' =>  2,
        'Y' =>  2,
        'C' =>  3,
        'Z' =>  3,
        _ => 0,
    }
}
