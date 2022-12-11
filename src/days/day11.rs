use std::{fs, str::FromStr, string::ParseError, cell::RefCell, rc::Rc};

const DAY: &str = "11";

#[derive(Debug, Clone)]
struct Expr {
    left: Option<i32>,
    right: Option<i32>,
    operator: char,
}

impl Expr {
    fn eval(&self) -> i32 {
        return match self.operator {
            '+' => self.left.unwrap() + self.right.unwrap(),
            '-' => self.left.unwrap() - self.right.unwrap(),
            '*' => self.left.unwrap() * self.right.unwrap(),
            '/' => self.left.unwrap() / self.right.unwrap(),
            _ => panic!("unsupported operator")
        }
    }
}

impl FromStr for Expr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn const_or_old(s: &str) -> Option<i32> {
            if s == "old" { return None; } 
            return Some(s.parse::<i32>().unwrap());
        }
        let mut values = s.split(" ").into_iter();
        let left: Option<i32> = const_or_old(values.next().unwrap().trim());
        let operator: char = values.next().unwrap().trim().chars().next().unwrap();
        let right: Option<i32> = const_or_old(values.next().unwrap().trim());
        Ok(Expr { left, right, operator })
    }
}

#[derive(Debug)]
struct Test {
    divisible: i32,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn eval(&self, v: i32) -> usize {
        if v % self.divisible == 0 {
            return self.if_true;
        }
        return self.if_false;
    }
}

fn to_test(lines: Vec<&str>) -> Test {
    let mut t = Test { divisible: 0, if_true: 0, if_false: 0 };

    for l in lines {
        let v = l.split_whitespace().last().unwrap().parse().unwrap();
        if l.contains("divisible") {
            t.divisible = v; 
        } 
        if l.contains("true") {
            t.if_true = v as usize; 
        } 
        if l.contains("false") {
            t.if_false = v as usize; 
        } 
    }
    return t;
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Expr,
    test: Test,
    held_item: Option<i32>,
}

impl Monkey {
    fn inspect(&mut self) {
        let item = self.items.pop().unwrap();
        let mut expr = self.operation.clone();

        if expr.left == None {
            expr.left = Some(item)
        }
        if expr.right == None {
            expr.right = Some(item)
        }
        self.held_item = Some(expr.eval() / 3);
    }

    fn target_monkey(&self) -> usize {
        return self.test.eval(self.held_item.unwrap())
    }

    fn throw(&mut self) -> i32 {
        let i = self.held_item.unwrap();
        self.held_item = None;
        return i;

    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1).into_iter();

        let mut items: Vec<i32> = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap().1.split(",").flat_map(|n| n.trim().parse::<i32>()).collect();

        let expr: Expr = lines.next().unwrap().split_once("=").unwrap().1.trim().parse().unwrap();

        let test: Test = to_test(lines.collect());

        println!("{:?}", items);
        println!("{:?}", expr);
        println!("{:?}", test);

        items.reverse();

        Ok(Monkey { items, operation: expr, test, held_item: None })
    }
}

fn monkey_business(monkies: &mut Vec<Rc<RefCell<Monkey>>>, rounds: usize) -> i32 {
    let mut inspections = vec![0; monkies.len()];

    for _ in 0..rounds {
        for (i, m) in monkies.iter().enumerate() {
            let mut cur = m.borrow_mut();
            for _ in 0..cur.items.len(){
                cur.inspect();
                let mut tm = monkies[cur.target_monkey()].borrow_mut();
                tm.items.push(cur.throw());
                inspections[i] += 1;
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    let mut mb = 1;
    for score in inspections.iter().take(2) {
        mb *= score; 
    }
    return mb;
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let mut input: Vec<Rc<RefCell<Monkey>>> = fs::read_to_string(test).unwrap().split("\n\n").map(|s| Rc::new(RefCell::new(s.parse::<Monkey>().unwrap()))).collect();

        
    println!("Dec {DAY}:");
    part_one(&mut input);
    println!();
}

fn part_one(monkies: &mut Vec<Rc<RefCell<Monkey>>>) {
    let result = monkey_business(monkies, 20);

    println!("Part one answer: {result}")
}
