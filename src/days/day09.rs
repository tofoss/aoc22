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
    let mut result: HashSet<Pos> = HashSet::new();
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    result.insert(tail.clone());

    for m in moves {
        head = match m.dir {
            'U' => Pos {
                x: head.x,
                y: head.y + m.n,
            },
            'D' => Pos {
                x: head.x,
                y: head.y - m.n,
            },
            'L' => Pos {
                x: head.x - m.n,
                y: head.y,
            },
            'R' => Pos {
                x: head.x + m.n,
                y: head.y,
            },
            _ => panic!("4d move not supported"),
        };
        let tail_visits = follow(&head, &tail);
        if tail_visits.len() > 0 {
            tail = tail_visits.last().unwrap().clone();
        }
        for ele in tail_visits {
            result.insert(ele);
        }
    }

    println!("Part one answer: {}", result.iter().count())
}

fn follow(head: &Pos, tail: &Pos) -> Vec<Pos> {
    let mut tail_visits: Vec<Pos> = Vec::new();
    let mut t = tail.clone();
    loop {
        match is_adj(head, &t) {
            false => {
                if head.x == t.x {
                    //move up or down
                    if head.y > t.y {
                        t = Pos { x: t.x, y: t.y + 1 };
                    }
                    if head.y < t.y {
                        t = Pos { x: t.x, y: t.y - 1 };
                    }
                } else if head.y == t.y {
                    //move left or right
                    if head.x > t.x {
                        t = Pos { x: t.x + 1, y: t.y };
                    }
                    if head.x < t.x {
                        t = Pos { x: t.x - 1, y: t.y };
                    }
                } else {
                    // up-right
                    if head.y > t.y && head.x > t.x {
                        t = Pos {
                            x: t.x + 1,
                            y: t.y + 1,
                        };
                    }
                    // down-left
                    if head.y < t.y && head.x < t.x {
                        t = Pos {
                            x: t.x - 1,
                            y: t.y - 1,
                        };
                    }
                    // up-left
                    if head.y > t.y && head.x < t.x {
                        t = Pos {
                            x: t.x - 1,
                            y: t.y + 1,
                        };
                    }
                    // down-right
                    if head.y < t.y && head.x > t.x {
                        t = Pos {
                            x: t.x + 1,
                            y: t.y - 1,
                        };
                    }
                }
                tail_visits.push(t.clone());
            }
            true => break,
        }
    }

    return tail_visits;
}

fn is_adj(head: &Pos, tail: &Pos) -> bool {
    if head.y == tail.y && head.x == tail.x {
        return true;
    }
    if head.y == tail.y && (head.x - tail.x).abs() == 1 {
        return true;
    }
    if head.x == tail.x && (head.y - tail.y).abs() == 1 {
        return true;
    }
    if (head.x - tail.x).abs() == 1 || (head.y - tail.y).abs() == 1 {
        if (head.x - tail.x).abs() == (head.y - tail.y).abs() {
            return true;
        }
    }
    return false;
}

fn part_two(moves: &Vec<Move>) {
    let mut result: HashSet<Pos> = HashSet::new();

    let mut rope = vec![
        Pos {
            x: 11,
            y: 5 
        };
        10
    ];
    result.insert(rope[9].clone());
    let mut grid = vec![vec!["."; 35]; 25];
    grid[rope[9].y as usize][rope[9].x as usize] = "#";

    println!("Visits:");
    for m in moves {
        let mut grid_mv = vec![vec!["."; 26]; 21];
        grid_mv[5][11] = "S";
        rope[0] = match m.dir {
            'U' => Pos {
                x: rope[0].x,
                y: rope[0].y + m.n,
            },
            'D' => Pos {
                x: rope[0].x,
                y: rope[0].y - m.n,
            },
            'L' => Pos {
                x: rope[0].x - m.n,
                y: rope[0].y,
            },
            'R' => Pos {
                x: rope[0].x + m.n,
                y: rope[0].y,
            },
            _ => panic!("4d move not supported"),
        };

        let strs = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

        grid_mv[rope[0].y as usize][rope[0].x as usize] = "H";

        for i in 1..(rope.len() - 1) {
            let visits = follow(&rope[i - 1], &rope[i]);
            if visits.len() > 0 {
                rope[i] = visits.last().unwrap().to_owned();
            }
            grid_mv[rope[i].y as usize][rope[i].x as usize] = strs[i];
        }

        let tail_visits = follow(&rope[8], &rope[9]);
        if tail_visits.len() > 0 {
            rope[9] = tail_visits.last().unwrap().clone();
        }
        grid_mv[rope[9].y as usize][rope[9].x as usize] = "T";

        println!("{:?}", m);
        print_grid(&mut grid_mv);
        println!();

        for ele in tail_visits {
            grid[ele.y as usize][ele.x as usize] = "#";
            result.insert(ele);
        }
    }

    print_grid(&mut grid);

    println!("Part two answer: {}", result.iter().count())
}

fn print_grid(grid: &mut Vec<Vec<&str>>) {
    grid.reverse();
    for i in 0..grid.len() {
        print!("{}: ", format!("{:02}", grid.len() - i));
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
