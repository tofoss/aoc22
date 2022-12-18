use std::{fs, str::FromStr, string::ParseError};

const DAY: &str = "14";

const MID: usize = 500;

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();

    let mut grid1 = grid_from(&input);
    let mut grid2 = grid_from(&input);
    place_floor(&mut grid2);

    let p1_limit = grid1.len() - 1;
    let p2_limit = 0;

    let units1 = pour_sand(&mut grid1, p1_limit);
    let units2 = pour_sand(&mut grid2, p2_limit);


    //print_grid(&grid2);
    println!("Dec {DAY}:");
    println!("Part one: {} units came to rest.", units1);
    println!("Part two: {} units came to rest.", units2 + 1);
    println!();
}

fn pour_sand(grid: &mut Vec<Vec<char>>, limit: usize) -> usize {
    let mut units = 0;
    loop {
        let p = simulate_flow(grid, Point { x: MID, y: 0 });
        if p.y == limit {
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

fn grid_from(input: &String) -> Vec<Vec<char>> {
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
        }
    } else if start.x > end.x {
        for i in end.x..start.x + 1 {
            grid[start.y][i] = '#';
        }
    } else if start.y < end.y {
        for i in start.y..end.y {
            grid[i][start.x] = '#';
        }
    } else if start.y > end.y {
        for i in end.y..start.y {
            grid[i][start.x] = '#';
        }
    }
}

fn place_floor(grid: &mut Vec<Vec<char>>) {
    let air = vec!['.'; MID * 2];
    let floor = vec!['#'; MID * 2];
    grid.push(air);
    grid.push(floor);
}
