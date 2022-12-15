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
    map[start.y][start.x].dist = 0;
    map[end.y][end.x].n = b'z';

    let mut paths: Vec<Point> = Vec::new();

        
    println!("Dec {DAY}:");

    spt(&mut map, &mut paths, start, end);
    retrace(&map, end, end);

    println!();
}

fn retrace(map: &Vec<Vec<Node>>, cur: Point, end: Point) {
    let s = map[cur.y][cur.x];
    if s.n == b'a' {
        println!("Found the A! {:?}", map[end.y][end.x].dist - map[cur.y][cur.x].dist);
        return;
    }
    let n: Vec<Point> = vec![s.up(), s.down(map.iter().count()), s.left(), s.right(map[0].len())]
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .filter(|c| map[c.y][c.x].dist == s.dist - 1)
        .collect();

    retrace(map, n[0], end)
}

fn spt(map: &mut Vec<Vec<Node>>, paths: &mut Vec<Point>, start: Point, end: Point) {
    if start == end {
        println!("Found the End! {:?}", map[end.y][end.x].dist);
        return
    }
    map[start.y][start.x].visited = true;
    let s = map[start.y][start.x];


    let neighbours: Vec<Point> = find_neighs(&s, map);
    if neighbours.is_empty() {
        let mut fresh_paths: Vec<Point> = paths.iter().filter(|p| !map[p.y][p.x].visited).map(|p| *p).collect();
        let mut closest = map[fresh_paths[0].y][fresh_paths[0].x];
        let mut idx = 0;
        for i in 0..fresh_paths.len() {
            if map[fresh_paths[i].y][fresh_paths[i].x].dist < closest.dist {
                closest = map[fresh_paths[i].y][fresh_paths[i].x];
                idx = i;
            }
        }
        fresh_paths.remove(idx);
        spt(map, &mut fresh_paths, Point {y: closest.y, x: closest.x}, end);
    } else {

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
        spt(map, paths, Point {y: closest.y, x: closest.x}, end);
    }

}

fn find_neighs(s: &Node, map: &Vec<Vec<Node>>) -> Vec<Point> {
    let n: Vec<Point> = vec![s.up(), s.down(map.iter().count()), s.left(), s.right(map[0].len())]
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .filter(|c| !map[c.y][c.x].visited)
        .collect();

    return n.iter().filter(|c| s.n >= map[c.y][c.x].n - 1).map(|c| *c).collect();
}

