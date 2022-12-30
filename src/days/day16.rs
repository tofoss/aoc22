use std::{collections::HashMap, fs};

use regex::Regex;

const DAY: &str = "16";

struct Graph {
    v: Vec<usize>,
    dist: Vec<Vec<usize>>,
    aa: usize,
}

impl Graph {
    fn max_flow(&mut self, cur_flow: usize, cur_pos: usize, dur: usize, nice_vs: &mut Vec<usize>) -> usize {
        if nice_vs.is_empty() || dur == 0 {
            return cur_flow;
        }

        let mut max = cur_flow;
        for v in nice_vs.iter().cloned() {
            let f = self.flow_of(v, cur_pos, dur);

            let mut ndir = 0;
            if dur - 1 >= self.dist[cur_pos][v] {
                ndir = dur - self.dist[cur_pos][v] - 1;
            }

            let mut next_nvs: Vec<usize> = nice_vs.iter().cloned().filter(|i| *i != v).collect();

            let mf = self.max_flow(cur_flow + f, v, ndir, &mut next_nvs);
            if mf > max {
                max = mf;
            }
        }

        return max;
    }

    fn flow_of(&self, loc: usize, cur_pos: usize, dur: usize) -> usize {
        let v = &self.v[loc];
        if dur - 1 <= self.dist[cur_pos][loc] {
            return 0;
        }
        return v * (dur - self.dist[cur_pos][loc] - 1);
    }
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();
    let mut graph = parse(input);

    let mut nice_vs = graph
        .v
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v > &0 {
                return Some(i);
            }
            return None;
        })
        .collect::<Vec<usize>>();

    let max_flow = graph.max_flow(0, graph.aa, 30, &mut nice_vs);

    println!();
    println!("Dec {DAY}:");
    println!("Part one -> Max flow: {}", max_flow);
}

fn parse(input: String) -> Graph {
    let vertice = input.lines().count();
    let mut positions: HashMap<String, usize> = HashMap::new();
    let mut v = Vec::new();
    let mut dist = vec![vec![usize::MAX / 2; vertice]; vertice];

    input.lines().enumerate().for_each(|(i, l)| {
        let re = Regex::new(r"Valve ([A-Z]+).*rate=(\d+)").unwrap();
        let cap = re.captures(l).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();
        let ver: usize = cap.get(2).unwrap().as_str().parse().unwrap();

        dist[i][i] = 0;
        positions.insert(name, i);
        v.push(ver)
    });

    input.lines().enumerate().for_each(|(i, l)| {
        let re = Regex::new(r"([A-Z][A-Z])").unwrap();
        for cap in re.captures_iter(l).skip(1) {
            let idx = positions.get(&cap[1].to_string()).unwrap();
            dist[i][*idx] = 1;
        }
    });

    for k in 0..vertice {
        for i in 0..vertice {
            for j in 0..vertice {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }

    return Graph { v, dist, aa: *positions.get(&String::from("AA")).unwrap()};
}
