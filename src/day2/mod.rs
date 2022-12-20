use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2_1() {
    let filename = "day2/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut player_score: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let v: Vec<u8> = line.bytes().collect();

        let rps = (v[0] - 64, v[2] - 87);

        player_score += rps.1 as usize;

        if rps.0 == rps.1 {
            player_score += 3;
            continue;
        }

        match rps {
            (3, 1) | (1, 2) | (2, 3) => player_score += 6,
            _ => { /*doesnt matter */ }
        }
    }
    println!("your score : {}", player_score);
}

//dont look at this its gross
pub fn day2_2() {
    let filename = "day2/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut player_score: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let v: Vec<u8> = line.bytes().collect();

        let mut rps = (v[0] - 64, v[2] - 87);
        let tmp;

        //AHHHHHHHHHHHHHHHHHH
        match rps {
            (1, 1) => tmp = 3,
            (2, 1) => tmp = 1,
            (3, 1) => tmp = 2,
            (1, 2) | (2, 2) | (3, 2) => tmp = rps.0,
            (1, 3) => tmp = 2,
            (2, 3) => tmp = 3,
            (3, 3) => tmp = 1,
            _ => panic!("invalid input!"),
        }

        rps.1 = tmp;

        player_score += rps.1 as usize;

        if rps.0 == rps.1 {
            player_score += 3;
            continue;
        }

        match rps {
            (3, 1) | (1, 2) | (2, 3) => player_score += 6,
            _ => { /* ignore */ }
        }
    }
    println!("{}", player_score);
}
