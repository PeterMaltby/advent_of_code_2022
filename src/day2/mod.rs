use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2_1() {
    let filename="day2/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut player_1_score :usize= 0;
    let mut player_2_score :usize= 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let v :Vec<u8>= line.bytes().collect();

        let rps = (v[0]-64, v[2]-87);

        player_1_score += rps.0 as usize;
        player_2_score += rps.1 as usize;

        if rps.0 == rps.1  {
            player_1_score += 3;
            player_2_score += 3;
            continue;
        }
        
        match rps {
            (1,3)|(2,1)|(3,2) => player_1_score += 6,
            (3,1)|(1,2)|(2,3) => player_2_score += 6,
            _ => panic!("invalid input!"),
        }

    }
    println!("your score : {}",player_2_score);
}


//dont look at this its gross
pub fn day2_2() {
    let filename="day2/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut player_1_score :usize= 0;
    let mut player_2_score :usize= 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let v :Vec<u8>= line.bytes().collect();

        let mut rps = (v[0]-64, v[2]-87);
        let mut tmp;

        //AHHHHHHHHHHHHHHHHHH
        match rps {
            (1,1) => tmp = 3,
            (2,1) => tmp = 1,
            (3,1) => tmp = 2,
            (1,2)|(2,2)|(3,2) => tmp = rps.0,
            (1,3) => tmp = 2,
            (2,3) => tmp = 3,
            (3,3) => tmp = 1,
            _ => panic!("invalid input!"),
        }

        rps.1 = tmp;

        player_1_score += rps.0 as usize;
        player_2_score += rps.1 as usize;

        if rps.0 == rps.1  {
            player_1_score += 3;
            player_2_score += 3;
            continue;
        }
        
        match rps {
            (1,3)|(2,1)|(3,2) => player_1_score += 6,
            (3,1)|(1,2)|(2,3) => player_2_score += 6,
            _ => panic!("invalid input!"),
        }
    }
    println!("your score : {}",player_2_score);
}
