use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day4_1() {
    let filename = "day4/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: isize = 0;
    //let v Vec<(isize,isize,isize,isize)> = new Vec;
    for line in reader.lines() {
        let line = line.unwrap();
        let v: Vec<isize> = line
            .split(|c| c == '-' || c == ',')
            .map(|s| s.parse().unwrap())
            .collect();

        if (v[0] >= v[2] && v[1] <= v[3]) || (v[0] <= v[2] && v[1] >= v[3]) {
            total += 1;
        }
    }

    println!("{}", total);
}

pub fn day4_2() {
    let filename = "day4/input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: isize = 0;
    //let v Vec<(isize,isize,isize,isize)> = new Vec;
    for line in reader.lines() {
        let line = line.unwrap();
        let v: Vec<isize> = line
            .split(|c| c == '-' || c == ',')
            .map(|s| s.parse().unwrap())
            .collect();

        if !((v[0] > v[2] && v[0] > v[3]) || (v[1] < v[3] && v[1] < v[2])) {
            total += 1;
        }
    }

    println!("{}", total);
}
