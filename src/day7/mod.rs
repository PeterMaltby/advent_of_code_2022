use std::fs::File;
use std::io::BufReader;

pub fn day7_1() {
    let filename = "day7/input.txt";

    // wow actual error handleing!
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("{:?}", error),
    };

    let mut reader = BufReader::new(file);

    for line in reader.lines() {

    }

    println!("not done yet");
}

pub fn day7_2() {
    let filename = "day7/input.txt";

    println!("not done yet!");
}
