use std::{fs, str::FromStr, string::ParseError, cmp::Ordering};

use regex::Regex;

const DAY: &str = "15";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"x=([-\d]+).*y=([-\d]+)").unwrap();
        let mut x = 0;
        let mut y = 0;
        

        for cap in re.captures_iter(s) {
            let xi = &cap[1];
            let yi = &cap[2];
            x = xi.parse().unwrap();
            y = yi.parse().unwrap();
        }

        Ok(Point { x, y })

    }
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    loc: Point,
    bec: Point,
}
 impl Sensor {
    fn rad(&self) -> i32 {
        return (self.loc.x - self.bec.x).abs() + (self.loc.y - self.bec.y).abs();
    }

    fn intersecting(&self, y: i32) -> Vec<Point> {
        if self.loc.y <= y {
            return self.intersect_down(y);
        }

        if self.loc.y > y {
            return self.intersect_up(y);
        }
         vec![]
    }

    fn intersect_down(&self, y: i32) -> Vec<Point> {
        let height = self.loc.y + self.rad();
        if height < y {
            return vec![];
        }
        let x1 = self.loc.x - (height - y) ;
        let x2 = height - y + self.loc.x;
        
        return vec![Point {x: x1, y }, Point {x: x2, y }]
    }

    fn intersect_up(&self, y: i32) -> Vec<Point> {
        let height = self.loc.y - self.rad();
        if  height > y {
            return vec![];
        }
        let x1 = self.loc.x - (height - y).abs() ;
        let x2 = (height - y).abs() + self.loc.x;
        
        return vec![Point {x: x1, y }, Point {x: x2, y }]
    }

     
 }

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    #[allow(unused_variables)]
    let input = fs::read_to_string(prod).unwrap();

    let sensors = parse(input.as_str());
    //let n = points_on_line(10, &sensors);
    let n = points_on_line(2_000_000, &sensors);
        
    println!("Dec {DAY}:");
    println!("{} positions where beacon is not", n);
}

fn points_on_line(y: i32, sensors: &Vec<Sensor>) -> i32 {
    println!("sensors intersecting y:");
    for s in sensors {
        let is = s.intersecting(y);
        if is.len() != 0 {
            println!("{:?} -> Rad: {}", s, s.rad());
            println!("Intersections: {:?}", is);
        }
    }

    let mut xs: Vec<Vec<Point>> = sensors.iter()
        .filter_map(|s| {
            if s.intersecting(y).len() == 0 {
                return None
            }
            return Some(s.intersecting(y))
        })
        .collect();
    
    xs.sort_by(|a, b| {
        if a[0].x < b[0].x {
            return Ordering::Less
        } else if a[0].x > b[0].x {
            return Ordering::Greater
        } else {
            return Ordering::Equal;
        }
    });

    let mut sum = 0;

    let mut cur = 0;
    let mut u = Vec::new();

    for i in 1..xs.len() {
        if xs[i][0].x > xs[cur][1].x {
            u.push(vec![xs[cur][0], xs[cur][1]]);
            cur = i;
        } 
        if xs[i][1].x > xs[cur][1].x {
            xs[cur][1].x = xs[i][1].x
        }
    }
    
    u.push(vec![xs[cur][0], xs[cur][1]]);

    for ele in u {
        println!("{:?}", ele);
        sum += ele[1].x - ele[0].x
    }
    return sum;
}

fn parse(input: &str) -> Vec<Sensor> {
    return input
        .lines()
        .map(|l| {
            let (s, b) = l.split_once(":").unwrap();
            Sensor {
                loc: s.parse().unwrap(),
                bec: b.parse().unwrap(),
            }
        })
        .collect();
}
