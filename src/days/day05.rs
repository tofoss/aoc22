use std::{fs, str::FromStr, string::ParseError};

const DAY: &str = "05";

#[derive(Debug)]
struct Instruction {
    mov: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let foo: Vec<_> = s.split_whitespace().flat_map(|a| a.parse::<usize>()).collect();

        Ok(Instruction { mov: foo[0], from: foo[1] - 1, to: foo[2] - 1 })
    }
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();
    let (stack_str, ins_str) = input.split_once("\n\n").unwrap();

    let stacks = to_stacks(stack_str);
    let instr: Vec<Instruction> = ins_str.lines().map(|s| s.parse::<Instruction>().unwrap()).collect();

    println!("Dec {DAY}:");
    part_one(&mut stacks.clone(), &instr);
    part_two(&mut stacks.clone(), &instr);
    println!();
}

fn part_one(stacks: &mut Vec<Vec<u8>>, instr: &Vec<Instruction>) {
    for ins in instr {
       exec(ins, stacks) 
    }
    print!("Part one: ");
    for s in stacks {
        let c = s.pop().unwrap() as char;
        print!("{}", c);
    }
    println!();
}

fn part_two(stacks: &mut Vec<Vec<u8>>, instr: &Vec<Instruction>) {
    for ins in instr {
       exec9001(ins, stacks) 
    }
    print!("Part two: ");
    for s in stacks {
        let c = s.pop().unwrap() as char;
        print!("{}", c);
    }
    println!();
}

fn exec9001(ins: &Instruction, stacks: &mut Vec<Vec<u8>>) {
    let mut tmp: Vec<u8> = Vec::new();
    for _ in 0..ins.mov {
        let crt = stacks.get_mut(ins.from).unwrap().pop().unwrap();
        tmp.push(crt)
    }
    tmp.reverse();
    stacks.get_mut(ins.to).unwrap().extend(tmp)
}

fn exec(ins: &Instruction, stacks: &mut Vec<Vec<u8>>) {
    for _ in 0..ins.mov {
        let crt = stacks.get_mut(ins.from).unwrap().pop().unwrap();
        stacks.get_mut(ins.to).unwrap().push(crt)
    }
}

fn to_stacks(stack_str: &str) -> Vec<Vec<u8>> {
    let matrix: Vec<Vec<u8>> = stack_str
        .lines()
        .map(|l| l.bytes().skip(1).step_by(4).collect())
        .collect();

    let mut stacks: Vec<Vec<u8>> = Vec::new();

    for _ in 0..matrix[0].len() {
        stacks.push(Vec::new());
    }

    for m in matrix {
       for (n, i) in m.iter().enumerate() {
           if i.is_ascii_alphabetic() {
               stacks[n].push(*i)
           }
       }
    }

    for s in stacks.iter_mut() {
        s.reverse()
    }
     return stacks;
}
