use std::{fs, str::FromStr, string::ParseError};

const DAY: &str = "10";

struct Machine {
    cpu: CPU,
    crt: CRT,
    interval: i32,
    emit_cycle: i32,
    cycle_state: i32,
    sum: i32,
}

impl Machine {
    fn exec(&mut self, op: &Op) {
        self.cycle(op.cycles());
        self.cpu.exec(op)
    }

    fn cycle(&mut self, cycles: i32) {
        if cycles > 0 {
            self.cycle_state += 1;
            self.crt.draw(self.cpu.reg_x);
            self.emit();
            self.cycle(cycles - 1);
        }
    }

    fn emit(&mut self) {
        if self.cycle_state == self.emit_cycle {
            self.sum += self.cpu.reg_x * self.cycle_state;
            self.emit_cycle += self.interval;
        }
    }
}

struct CPU {
    reg_x: i32,
}

impl CPU {
    fn exec(&mut self, op: &Op) {
        if op.t == OpType::ADDX {
            self.reg_x += op.value.unwrap();
        }
    }
}

struct CRT {
    width: i32,
    cur_x: i32,
    cur_y: i32,
}

impl CRT {
    fn draw(&mut self, reg_x: i32) {
        let sprite = sprite(reg_x, self.width);
        print!("{}", sprite[self.cur_x as usize]);
        self.move_cur()
    }

    fn move_cur(&mut self) {
        self.cur_x += 1;
        if self.cur_x == self.width {
            self.cur_x = 0;
            self.cur_y += 1;
            println!()
        }
    }
}

fn sprite(reg_x: i32, max: i32) -> Vec<&'static str> {
    let mut s: Vec<&str> = Vec::new();
    for i in 0..max {
        if reg_x - 1 == i || reg_x == i || reg_x + 1 == i {
            s.push("#");
        } else {
            s.push(".");
        }
    }
    return s;
    
}

#[derive(Debug, PartialEq, Eq)]
enum OpType {
    ADDX,
    NOOP,
}

#[derive(Debug)]
struct Op {
    t: OpType,
    value: Option<i32>,
}

impl Op {
    fn cycles(&self) -> i32 {
        match self.t {
            OpType::ADDX => 2,
            OpType::NOOP => 1,
        }
    }
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split(" ");
        match spl.next().unwrap() {
            "noop" => Ok(Op {
                t: OpType::NOOP,
                value: None,
            }),
            "addx" => Ok(Op {
                t: OpType::ADDX,
                value: Some(spl.last().unwrap().parse::<i32>().unwrap()),
            }),
            _ => panic!("Unknown operation {}", spl.next().unwrap()),
        }
    }
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input: Vec<Op> = fs::read_to_string(prod)
        .unwrap()
        .trim()
        .split("\n")
        .map(|op| op.parse::<Op>().unwrap())
        .collect();

    println!("Dec {DAY}:");
    part_one(&input);
    println!();
}

fn part_one(input: &Vec<Op>) {
    let crt = CRT { width: 40, cur_x: 0, cur_y: 0 };
    let cpu = CPU {reg_x: 1};
    let mut machine = Machine { cpu, crt, interval: 40, emit_cycle: 20, cycle_state: 0, sum: 0 };
    for op in input {
        machine.exec(op);
    }

    println!("Part one answer: {}", machine.sum)
}
