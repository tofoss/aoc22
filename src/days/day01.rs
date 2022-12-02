use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/day01-1.txt")
        .expect("file not found");

    println!("Dec 01:");
    part_one(&input);
    part_two(&input);
    println!();
}

fn part_one(input: &String) {
    let mut big_elf = 0;
    let mut cur = 0;
    input.split("\n").for_each(|g| {
        if !g.is_empty() {
            cur += g.parse::<u32>().unwrap();
        } else {
            big_elf = if cur > big_elf { cur } else { big_elf };
            cur = 0;
        }
    });
    println!("Part one answer: {big_elf}")
}


fn part_two(input: &String) {
    let mut top_three:[u32; 3] = [0, 0, 0];
    let mut cur = 0;
    input.split("\n").for_each(|g| {
        if !g.is_empty() {
            cur += g.parse::<u32>().unwrap();
        } else {
            comp_top_three(&mut top_three, cur);
            cur = 0;
        }
    });

    let mut sum = 0;
    for i in 0..top_three.len() {
        sum += top_three[i]
    }

    println!("Part two answer: {sum}")
}

fn comp_top_three(top_three: &mut [u32], elf: u32) {
    let mut n = elf;
    for i in 0..top_three.len() {
        if n > top_three[i] {
            let swp = top_three[i];
            top_three[i] = n;
            n = swp;
        }
    }
}
