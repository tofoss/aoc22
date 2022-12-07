use std::{cell::RefCell, fs, rc::Rc};

const DAY: &str = "07";

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<Node>>>,
}

fn new_node(name: String, size: usize) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node {
        name,
        size,
        children: vec![],
    }))
}

pub fn solve() {
    #[allow(unused_variables)]
    let test = format!("input/day{DAY}-test.txt");
    #[allow(unused_variables)]
    let prod = format!("input/day{DAY}.txt");

    let input = fs::read_to_string(prod).unwrap();
    let file_system: FileSystem = to_fs(input.trim().split("\n").collect::<Vec<_>>());

    //file_system.recurse(&file_system.root);

    println!("Dec {DAY}:");
    part_one(&file_system);
    part_two(&file_system);
    println!();
}

const CD: &str = "$ cd";

struct FileSystem {
    root: Rc<RefCell<Node>>,
    stack: Vec<Rc<RefCell<Node>>>,
}

impl FileSystem {
    fn cd(&mut self, dir: &str) {
        if dir == "/" {
            self.stack.clear();
            self.stack.push(Rc::clone(&self.root))
        } else if dir == ".." {
            self.stack.pop();
        } else {
            let node = new_node(dir.to_string(), 0);
            self.stack
                .last_mut()
                .unwrap()
                .borrow_mut()
                .children
                .push(Rc::clone(&node));
            self.stack.push(node);
        }
    }

    fn total_size(&self, node: &Rc<RefCell<Node>>) -> usize {
        let mut total = node.borrow().size;
        for child in &node.borrow().children {
            total += self.total_size(child);
        }

        return total;
    }

    fn dir_sizes(&self, node: &Rc<RefCell<Node>>, max_sixe: usize) -> usize {
        let mut ds = 0;

        for child in &node.borrow().children {
            if child.borrow().size == 0 {
                let ts = self.total_size(child);
                if ts <= max_sixe {
                    ds += ts
                }
                ds += self.dir_sizes(child, max_sixe);
            }
        }
        return ds;
    }


    fn smallest_free(&self, node: &Rc<RefCell<Node>>, min_size: usize, closest: usize) -> usize {
        if node.borrow().size != 0 {
            return 0;
        }

        let mut cur = self.total_size(node);

        if cur <= min_size || cur > closest {
            cur = closest;
        }
        

        for child in &node.borrow().children {
            if child.borrow().size == 0 {
                cur = self.smallest_free(child, min_size, cur);
            }
        }
        return cur;
    }


    fn ls_file(&mut self, file: &str) {
        let (size, name) = file.split_once(" ").unwrap();
        let node = new_node(name.to_string(), size.parse::<usize>().unwrap());
        self.stack
            .last_mut()
            .unwrap()
            .borrow_mut()
            .children
            .push(Rc::clone(&node));
    }
}

fn to_fs(output: Vec<&str>) -> FileSystem {
    let mut sys = FileSystem {
        root: new_node(String::from("/"), 0),
        stack: vec![],
    };
    for op in output {
        if op.starts_with(CD) {
            sys.cd(op.replace(CD, "").trim());
        } else if op.chars().next().unwrap().is_numeric() {
            sys.ls_file(op);
        }
    }
    return sys;
}

fn part_one(sys: &FileSystem) {
    let result: usize = sys.dir_sizes(&sys.root, 100_000);

    println!("Part one answer: {result}")
}

fn part_two(sys: &FileSystem) {
    let total_size = sys.total_size(&sys.root);
    let needed_space = (total_size + 30_000_000) % 70_000_000;
    let result: usize = sys.smallest_free(&sys.root, needed_space, total_size);

    println!("Part two answer: {result}")
}
