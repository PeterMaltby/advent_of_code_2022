use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new(input: BufReader<File>) -> Self {
        let mut tmp: Vec<String> = Vec::new();

        for line in input.lines() {
            let mut line = line.unwrap();

            if line.is_empty() {
                break;
            }

            let mut c = -1;
            line.retain(|_| {
                c += 1;
                return c % 4 == 1;
            });
            tmp.push(line);
        }

        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); tmp[tmp.len() - 1].len()];

        for line in tmp.iter().rev() {
            for (y, c) in line.chars().enumerate() {
                if c != ' ' {
                    stacks[y].push(c);
                }
            }
        }

        Self { stacks }
    }

    pub fn print(&self) {
        for stack in &self.stacks {
            println!("{:?}", stack);
        }
    }

    pub fn do_instruct(&mut self, from: usize, to: usize, num: usize) {
        for _ in 0..num {
            let tmp = self.stacks[from - 1].pop();
            self.stacks[to - 1].push(tmp.unwrap());
        }
    }

    pub fn do_instruct_9001(&mut self, from: usize, to: usize, num: usize) {
        let mut crane_stack: Vec<char> = Vec::new();
        for _ in 0..num {
            crane_stack.push(self.stacks[from - 1].pop().unwrap());
        }
        for _ in 0..crane_stack.len() {
            self.stacks[to - 1].push(crane_stack.pop().unwrap());
        }
    }
}

pub fn day5_1() {
    let filename = "day5/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut stacks: Stacks = Stacks::new(reader);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (c, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.len() > 4 {
            if &line[0..4] == "move" {
                let v: Vec<&str> = line.split(|c| c == ' ').collect();
                stacks.do_instruct(
                    v[3].parse().unwrap(),
                    v[5].parse().unwrap(),
                    v[1].parse().unwrap(),
                );
            }
        }
    }

    stacks.print();
}

pub fn day5_2() {
    let filename = "day5/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut stacks: Stacks = Stacks::new(reader);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (c, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.len() > 4 {
            if &line[0..4] == "move" {
                let v: Vec<&str> = line.split(|c| c == ' ').collect();
                stacks.do_instruct_9001(
                    v[3].parse().unwrap(),
                    v[5].parse().unwrap(),
                    v[1].parse().unwrap(),
                );
            }
        }
    }

    stacks.print();
}
