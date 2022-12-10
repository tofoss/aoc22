use std::{collections::HashSet, fs, str::FromStr, string::ParseError};

const DAY: &str = "09";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Move {
    dir: char,
    n: i64,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, n) = s.split_once(" ").unwrap();
        Ok(Move {
            dir: dir.chars().next().unwrap(),
            n: n.parse::<i64>().unwrap(),
        })
    }
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let test2 = format!("input/day{DAY}-test2.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let moves: Vec<Move> = fs::read_to_string(test2)
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.parse::<Move>().unwrap())
        .collect();

    println!("Dec {DAY}:");
    part_one(&moves);
    part_two(&moves);
    println!();
}

fn part_one(moves: &Vec<Move>) { 
    let mut rope = vec![ Pos { x: 0, y: 0 }; 2 ];
    let result: usize = move_knots(&mut rope, moves);
    println!("Part one answer: {}", result)
}

fn part_two(moves: &Vec<Move>) {
    let mut rope = vec![ Pos { x: 0, y: 0 }; 10 ]; 
    let result: usize = move_knots(&mut rope, moves);
    println!("Part two answer: {}", result)
}

fn move_knots(rope: &mut Vec<Pos>, moves: &Vec<Move>) -> usize {
    let mut visits: Vec<Pos> = Vec::new();
    for m in moves {
        let tail_visits = match m.dir {
            'R' => mv(rope, m.n, &mut|rope: &mut Vec<Pos>| rope[0].x += 1),
            'L' => mv(rope, m.n, &mut|rope: &mut Vec<Pos>| rope[0].x -= 1),
            'U' => mv(rope, m.n, &mut|rope: &mut Vec<Pos>| rope[0].y += 1),
            'D' => mv(rope, m.n, &mut|rope: &mut Vec<Pos>| rope[0].y -= 1),
            _ => panic!("4D moves are not supported"),
        };
        visits.extend(tail_visits);
    }
    //print_grid(&visits);
    let result: HashSet<&Pos> = visits.iter().collect();
    return result.iter().count();
}

fn mv<F: FnMut(&mut Vec<Pos>) -> ()>(rope: &mut Vec<Pos>, n: i64, mv_head: &mut F) -> Vec<Pos> {
    let mut visits: Vec<Pos> = Vec::new();
    for _ in 0..n {
        mv_head(rope);
        mv_tail(rope);
        visits.push(rope[rope.len()-1].clone());
    }
    return visits;
}

fn mv_tail(rope: &mut Vec<Pos>) {
    for i in 1..rope.len() {
        if (rope[i - 1].x - rope[i].x).abs() < 2 && (rope[i - 1].y - rope[i].y).abs() < 2 {
            continue;
        }
        if rope[i - 1].x > rope[i].x {
            rope[i].x += 1
        }
        if rope[i - 1].x < rope[i].x {
            rope[i].x -= 1
        }
        if rope[i - 1].y > rope[i].y {
            rope[i].y += 1
        }
        if rope[i - 1].y < rope[i].y {
            rope[i].y -= 1
        }
    }
}

fn print_grid(visits: &Vec<Pos>) {
    let mut grid = vec![vec!["."; 55]; 30];
    for v in visits {
        grid[(v.y + 11) as usize][(v.x + 11) as usize] = "#";
    }

    grid.reverse();
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

