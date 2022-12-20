use std::fs::File as FsFile;
use std::io::{BufRead, BufReader};

struct Cpu {
    reg: isize,
    clock: isize,
    signal_sum : isize,
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu { reg: 1, clock: 0, signal_sum: 0 }
    }

    fn  tick(mut self) -> Cpu { 
        self.clock += 1;
//        println!("[{}] x:{} s:{}",self.clock,self.reg, self.signal_sum);

        if (self.clock + 20) % 40 == 0 {
            self.signal_sum += self.clock * self.reg;
        }

        self
    }

    pub fn addx(mut self, i :isize) -> Cpu {
        self = self.tick();
        self = self.tick();
        self.reg += i;
        self
    }

    pub fn noop(mut self) -> Cpu {
        self = self.tick();
        self
    }

    pub fn signal_sum(self) -> isize { self.signal_sum }
}

pub fn day10_1() {
    let filename = "day10/input.txt";

    let file = FsFile::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut cpu = Cpu::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let words = line.split(' ').collect::<Vec<&str>>();
        
        match words[0] {
            "addx" =>  cpu = cpu.addx(words[1].parse().unwrap()),
            "noop" => cpu = cpu.noop(),
            _ => panic!("not working!"),
        }
    }

    println!("{}", cpu.signal_sum());
}

pub fn day10_2() {
    println!("not done yet!");
}
