use std::fs;

pub fn day6_1() {
    let filename = "day6/input.txt";

    let stream = fs::read_to_string(filename).expect("cant read file");
    let stream = stream.as_bytes();

    for i in 0..stream.len() - 4 {
        let mut unique = true;
        for x in 0..3 {
            for y in x + 1..4 {
                if stream[i + x] == stream[i + y] {
                    unique = false;
                }
            }
        }

        if unique {
            println!("{}", i + 4);
            break;
        }
    }
}

pub fn day6_2() {
    let filename = "day6/input.txt";

    let stream = fs::read_to_string(filename).expect("cant read file");
    let stream = stream.as_bytes();

    for i in 0..stream.len() - 14 {
        let mut unique = true;
        for x in 0..13 {
            for y in x + 1..14 {
                if stream[i + x] == stream[i + y] {
                    unique = false;
                }
            }
        }

        if unique {
            println!("{}", i + 14);
            break;
        }
    }
}
