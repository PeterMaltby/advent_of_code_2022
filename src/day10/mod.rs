use std::fs::File as FsFile;
use std::io::{BufRead, BufReader};

pub fn day10_1() {
    let filename = "day10/testInput.txt";

    let file = FsFile::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        println!("{}", line);
    }

    println!("not done yet!");
}

pub fn day10_2() {
    println!("not done yet!");
}
