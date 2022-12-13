use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day8_1() {
    let filename="day8/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<u32>> = vec![Vec::new(); 0];
    
    for line in reader.lines() {
        let line = line.unwrap();
        forest.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    //for f in &forest { println!("{:?}",f); }

    let mut count = 0;
    for (x,row) in forest.iter().enumerate() {
        for (y,tree) in row.iter().enumerate() {
            if x == 0 || y == 0 || x+1 == forest[0].len() || y+1 == forest.len() { count +=1; continue }
                
            //assumes visable and - if not
            let mut angles_count = 4;
            //left
            for c in 0..y {
                if tree <= &forest[x][c] { angles_count -= 1; break;  }
            }

            //right
            for c in y+1..forest[y].len() {
                if tree <= &forest[x][c] { angles_count -= 1; break;  }
            }

            for c in 0..x {
                if tree <= &forest[c][y] { angles_count -=1;break;}
            }

            for c in x+1..forest.len() {
                if tree <= &forest[c][y] { angles_count -=1;break;}
            }

            if angles_count > 0 { count += 1; }

        }
    }

    println!("{}",count);

}



pub fn day8_2() {
    println!("unsolved!");
}

