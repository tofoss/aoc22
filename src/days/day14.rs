use std::{fs, str::FromStr, string::ParseError};

const DAY: &str = "14";

const MID: usize = 500;

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();

    let mut grid = grid_from(input);

    let units = pour_sand(&mut grid);


    print_grid(&grid);
    println!("Dec {DAY}:");
    println!("{} units came to rest.", units);
    println!();
}

fn pour_sand(grid: &mut Vec<Vec<char>>) -> usize {
    let mut units = 0;
    loop {
        let p = simulate_flow(grid, Point { x: MID, y: 0 });
        if p.y >= grid.len() - 1 {
            break;
        }
        units += 1;
    }
    return units;
}

fn simulate_flow(grid: &mut Vec<Vec<char>>, p: Point) -> Point {
    if p.y >= grid.len() - 1 {
        return p;
    }
    if grid[p.y + 1][p.x] == '.' {
        return simulate_flow(grid, Point { y: p.y + 1, x: p.x });
    } else if grid[p.y + 1][p.x - 1] == '.' {
        return simulate_flow( grid, Point { y: p.y + 1, x: p.x - 1, },);
    } else if grid[p.y + 1][p.x + 1] == '.' {
        return simulate_flow( grid, Point { y: p.y + 1, x: p.x + 1, },);
    } else {
        grid[p.y][p.x] = 'o';
        return p;
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in MID - 20..MID + 50 {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

fn grid_from(input: String) -> Vec<Vec<char>> {
    let rock_formations: Vec<Vec<Point>> = input
        .lines()
        .map(|s| {
            s.trim()
                .split(" -> ")
                .map(|p| p.trim().parse::<Point>().unwrap())
                .collect()
        })
        .collect();

    let max_y = rock_formations
        .iter()
        .flatten()
        .max_by_key(|z| z.y)
        .unwrap();
    let mut grid = vec![vec!['.'; MID * 2]; max_y.y + 1];

    for rocks in rock_formations {
        for i in 1..rocks.len() {
            place_rocks(&rocks[i - 1], &rocks[i], &mut grid);
        }
    }
    return grid;
}

fn place_rocks(start: &Point, end: &Point, grid: &mut Vec<Vec<char>>) {
    if start.x < end.x {
        for i in start.x..end.x + 1 {
            grid[start.y][i] = '#';
            println!("placing right: {}, {}", start.y, i);
        }
    } else if start.x > end.x {
        for i in end.x..start.x + 1 {
            grid[start.y][i] = '#';
            println!("placing left: {}, {}", start.y, i);
        }
    } else if start.y < end.y {
        for i in start.y..end.y {
            grid[i][start.x] = '#';
            println!("placing down: {}, {}", i, start.x);
        }
    } else if start.y > end.y {
        for i in end.y..start.y {
            grid[i][start.x] = '#';
            println!("placing down: {}, {}", i, start.x);
        }
    }
}
