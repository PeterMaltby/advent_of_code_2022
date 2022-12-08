use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn day3_1() {
	let filename="day3/input.txt";
	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);

        let mut tmp :isize= 0;

	for (_index, line) in reader.lines().enumerate() {
		let line = line.unwrap();
//                let v :Vec<u8>= line.bytes().map(|x| if x > 91 {x - 96 } else {x - 38}).collect();

                let length = line.len()/2;

                let a = &line[0..length];
                let b = &line[length..line.len()];
                
                for charecter in b.chars() {
                    if a.contains(charecter) {
                        let c = charecter as isize;
                        let c2 = if c > 91 {c - 96 } else {c - 38};
                        tmp += c2;
                        break;
                        }
                }
	}
        println!("{}", tmp);
}



pub fn day3_2() {
	let filename="day3/input.txt";
	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);

        let mut tmp :isize= 0;

	for (_index, line) in reader.lines().enumerate() {
		let line = line.unwrap();
//                let v :Vec<u8>= line.bytes().map(|x| if x > 91 {x - 96 } else {x - 38}).collect();

                let length = line.len()/2;

                let a = &line[0..length];
                let b = &line[length..line.len()];
                
                for charecter in b.chars() {
                    if a.contains(charecter) {
                        let c = charecter as isize;
                        let c2 = if c > 91 {c - 96 } else {c - 38};
                        tmp += c2;
                        break;
                        }
                }
	}
        println!("{}", tmp);
}
