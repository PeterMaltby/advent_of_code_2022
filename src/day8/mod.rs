use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day8_1() {
    let filename = "day8/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<u32>> = vec![Vec::new(); 0];

    for line in reader.lines() {
        let line = line.unwrap();
        forest.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    //for f in &forest { println!("{:?}",f); }

    let mut count = 0;
    for (x, row) in forest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            //assumes visable and - if not -1
            let mut angles_count = 4;
            //left
            for c in 0..y {
                if tree <= &forest[x][c] {
                    angles_count -= 1;
                    break;
                }
            }

            //right
            for c in y + 1..forest[y].len() {
                if tree <= &forest[x][c] {
                    angles_count -= 1;
                    break;
                }
            }

            for c in 0..x {
                if tree <= &forest[c][y] {
                    angles_count -= 1;
                    break;
                }
            }

            for c in x + 1..forest.len() {
                if tree <= &forest[c][y] {
                    angles_count -= 1;
                    break;
                }
            }

            if angles_count > 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

pub fn day8_2() {
    let filename = "day8/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<u32>> = vec![Vec::new(); 0];
    let mut scores: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        forest.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    for (x, row) in forest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            let mut score = 0;
            //left
            for c in (0..y).rev() {
                score += 1;
                if tree <= &forest[x][c] {
                    break;
                }
            }

            let mut tmp = 0;
            //right
            for c in y + 1..forest[y].len() {
                tmp += 1;
                if tree <= &forest[x][c] {
                    break;
                }
            }
            score = score * tmp;

            let mut tmp = 0;
            for c in (0..x).rev() {
                tmp += 1;
                if tree <= &forest[c][y] {
                    break;
                }
            }
            score = score * tmp;

            let mut tmp = 0;
            for c in x + 1..forest.len() {
                tmp += 1;
                if tree <= &forest[c][y] {
                    break;
                }
            }

            scores.push(score * tmp);
        }
    }

    println!("{}", scores.iter().max().unwrap());
}
