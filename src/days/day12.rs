use std::fs;

const DAY: &str = "12";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    n: u8,
    y: usize,
    x: usize,
    dist: usize,
    visited: bool,
}

impl Node {
    fn up(&self) -> Option<Point> {
        match self.y {
            0 => None,
            _ => Some(Point{y: self.y-1, x: self.x})
        }
    }
    fn down(&self, limit: usize) -> Option<Point> {
        if self.y >= limit - 1 {
            return None;
        }
        return Some(Point{y: self.y+1, x: self.x})
    }
    fn left(&self) -> Option<Point>{
        match self.x {
            0 => None,
            _ => Some(Point{y: self.y, x: self.x-1})
        }
    }
    fn right(&self, limit: usize) -> Option<Point> {
        if self.x >= limit - 1 {
            return None;
        }
        return Some(Point{y: self.y, x: self.x+1});
    }
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input: Vec<Vec<u8>> = fs::read_to_string(prod)
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| s.bytes().collect())
        .collect();

    let mut start = Point { y: 0, x: 0 };
    let mut end = Point { y: 0, x: 0 };
    let mut map: Vec<Vec<Node>> = Vec::new();
    for (y, r) in input.iter().enumerate() {
        let mut row: Vec<Node> = Vec::new();
        for (x, c) in r.iter().enumerate() {
            if *c == b'S' {
                start = Point {y, x };
            }
            if *c == b'E' {
                end = Point {y, x };
            }
            row.push(Node {n: *c, y, x, dist: usize::MAX, visited: false });
        }
        map.push(row)
    }
    map[start.y][start.x].n = b'a';
    map[end.y][end.x].n = b'z';
    map[end.y][end.x].dist = 0;

    let mut paths: Vec<Point> = Vec::new();

        
    println!("Dec {DAY}:");

    spt2(&mut map, &mut paths, end, b'a', Point {x: 0, y: 0});

    println!();
}

fn spt2(map: &mut Vec<Vec<Node>>, paths: &mut Vec<Point>, cur: Point, end: u8, nearest: Point) {
    map[cur.y][cur.x].visited = true;
    let s = map[cur.y][cur.x];

    let mut near = nearest;

    if s.n == end {
        if s.dist < map[nearest.y][nearest.x].dist {
            near = Point {y: s.y, x: s.x};
        }

        println!("Found the A! {:?}", s);
    }

    let neighbours: Vec<Point> = find_neighs2(&s, map);
    if neighbours.is_empty() && paths.is_empty() {
        println!("Found the closest A! {:?}", map[near.y][near.x]);
        return
    }

    if neighbours.is_empty() {
        return backtrack2(map, paths, end, near)
    } 
    let mut closest = map[neighbours[0].y][neighbours[0].x];

    for n in &neighbours {
        if map[n.y][n.x].dist > s.dist + 1 {
            map[n.y][n.x].dist = s.dist + 1;
        }
        if map[n.y][n.x].dist < closest.dist {
            closest = map[n.y][n.x]
        }
        paths.push(*n);
    }
    spt2(map, paths, Point {y: closest.y, x: closest.x}, end, near);
}

fn backtrack2(map: &mut Vec<Vec<Node>>, paths: &mut Vec<Point>, end: u8, nearest: Point) {
    let mut unvisited: Vec<Point> = paths.iter().filter(|p| !map[p.y][p.x].visited).map(|p| *p).collect();
    let mut closest = map[unvisited[0].y][unvisited[0].x];
    let mut idx = 0;
    for i in 0..unvisited.len() {
        if map[unvisited[i].y][unvisited[i].x].dist < closest.dist {
            closest = map[unvisited[i].y][unvisited[i].x];
            idx = i;
        }
    }
    unvisited.remove(idx);

    spt2(map, &mut unvisited, Point {y: closest.y, x: closest.x}, end, nearest);
}

fn find_neighs2(s: &Node, map: &Vec<Vec<Node>>) -> Vec<Point> {
    let n: Vec<Point> = vec![s.up(), s.down(map.iter().count()), s.left(), s.right(map[0].len())]
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .filter(|c| !map[c.y][c.x].visited)
        .collect();

    return n.iter().filter(|c| s.n <= map[c.y][c.x].n + 1).map(|c| *c).collect();
}
