use std::fs::File as FsFile;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn day9_1() {
    let filename = "day9/input.txt";

    let file = FsFile::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut h_pos = (0,0);
    let mut t_pos = (0,0);
    let mut t_positions = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split(' ').collect::<Vec<&str>>();
        
        let repeat = words[1].parse().unwrap();
        
        for _ in 0..repeat {
            match words[0] {
                "U" => h_pos = (h_pos.0,h_pos.1+1),
                "D" => h_pos = (h_pos.0,h_pos.1-1),
                "R" => h_pos = (h_pos.0+1,h_pos.1),
                "L" => h_pos = (h_pos.0-1,h_pos.1),
                _ => panic!("paniced!"),
            }

            let mut move_allowed = true;
            for x in -1..2 {
                for y in -1..2 {
                    if t_pos == (h_pos.0+x,h_pos.1+y) {move_allowed = false;}
                }
            }

            if move_allowed {
                if h_pos.0 > t_pos.0 { t_pos.0 += 1; }
                if h_pos.0 < t_pos.0 { t_pos.0 -= 1; }
                if h_pos.1 > t_pos.1 { t_pos.1 += 1; }
                if h_pos.1 < t_pos.1 { t_pos.1 -= 1; }
            }
            
            t_positions.insert(t_pos);
        }

    }

    println!("{:?}", t_positions.len());
}

pub fn day9_2() {
    let filename = "day9/input.txt";

    let file = FsFile::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rope = vec!((0,0);10);
    let mut t_positions = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split(' ').collect::<Vec<&str>>();
        
        let repeat = words[1].parse().unwrap();
        
        for _ in 0..repeat {
            match words[0] {
                "U" => rope[0] = (rope[0].0,rope[0].1+1),
                "D" => rope[0] = (rope[0].0,rope[0].1-1),
                "R" => rope[0] = (rope[0].0+1,rope[0].1),
                "L" => rope[0] = (rope[0].0-1,rope[0].1),
                _ => panic!("paniced!"),
            }

            for i in 1..10 {

                let mut move_allowed = true;
                for x in -1..2 {
                    for y in -1..2 {
                        if rope[i] == (rope[i-1].0+x,rope[i-1].1+y) {move_allowed = false;}
                    }
                }

                if move_allowed {
                    if rope[i-1].0 > rope[i].0 { rope[i].0 += 1; }
                    if rope[i-1].0 < rope[i].0 { rope[i].0 -= 1; }
                    if rope[i-1].1 > rope[i].1 { rope[i].1 += 1; }
                    if rope[i-1].1 < rope[i].1 { rope[i].1 -= 1; }
                }
            }

            let tmp = rope[9];
            t_positions.insert((rope[9].0,rope[9].1));
        }

    }
    println!("{:?}", t_positions.len());
}
