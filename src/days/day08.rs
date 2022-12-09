use std::fs;

const DAY: &str = "08";

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();
    let forest: Vec<Vec<u8>> = input
        .trim()
        .split("\n")
        .map(|s| s.as_bytes().into())
        .collect();

    println!("Dec {DAY}:");
    part_one(&forest);
    part_two(&forest);
    println!();
}

fn part_one(forest: &Vec<Vec<u8>>) {
    let mut result: usize = 0;
    let mut visibility = vec![vec![true; forest.first().unwrap().len()]; forest.len()];

    for i in 1..forest.len() - 1 {
        for j in 1..forest[i].len() - 1 {
            visibility[i][j] = is_visible(i, j, forest);
        }
    }

    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            if visibility[i][j] {
                result += 1;
                print!("{}", forest[i][j] - 48);
            } else {
                print!("_");
            }
        }
        println!();
    }

    println!("Part one answer: {result}")
}

fn is_visible(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> bool {
    return is_visible_south(y, x, forest)
        || is_visible_north(y, x, forest)
        || is_visible_east(y, x, forest)
        || is_visible_west(y, x, forest);
}

fn is_visible_north(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> bool {
    for i in (0..y).rev() {
        if forest[i][x] >= forest[y][x] {
            return false;
        }
    }
    return true;
}

fn is_visible_south(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> bool {
    for i in y + 1..forest.len() {
        if forest[i][x] >= forest[y][x] {
            return false;
        }
    }
    return true;
}

fn is_visible_west(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> bool {
    for i in x + 1..forest[y].len() {
        if forest[y][i] >= forest[y][x] {
            return false;
        }
    }
    return true;
}

fn is_visible_east(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> bool {
    for i in (0..x).rev() {
        if forest[y][i] >= forest[y][x] {
            return false;
        }
    }
    return true;
}

fn part_two(forest: &Vec<Vec<u8>>) {
    let mut result: usize = 0;

    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            let sc: usize = scene_score(i, j, forest);
            if sc > result {
                result = sc;
            } 
        }
    }
    println!("Part two answer: {result}")
}

fn scene_score(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> usize {
    return scenic_south(y, x, forest)
        * scenic_north(y, x, forest)
        * scenic_east(y, x, forest)
        * scenic_west(y, x, forest);
}

fn scenic_north(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    for i in (0..y).rev() {
        score += 1;
        if forest[i][x] >= forest[y][x] {
            break;
        }
    }
    return score;
}

fn scenic_south(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    for i in y + 1..forest.len() {
        score += 1;
        if forest[i][x] >= forest[y][x] {
            break;
        }
    }
    return score;
}

fn scenic_west(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    for i in x + 1..forest[y].len() {
        score += 1;
        if forest[y][i] >= forest[y][x] {
            break;
        }
    }
    return score;
}

fn scenic_east(y: usize, x: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    for i in (0..x).rev() {
        score += 1;
        if forest[y][i] >= forest[y][x] {
            break
        }
    }
    return score;
}
