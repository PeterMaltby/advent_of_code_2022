use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::izip;

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

    let mut elf= 0;
    let mut total= 0;

    // 3 arrays of 52 booleans
    let mut elfs_arr :[[bool; 52]; 3] = [[false;52];3];

	for (_index, line) in reader.lines().enumerate() {
		let line = line.unwrap();
        let v :Vec<u8>= line.bytes().map(|x| if x > 91 {x - 96 } else {x - 38}).collect();
        elfs_arr[elf] = [false; 52];
        for item in v {
            let index = item as usize;
            elfs_arr[elf][index-1]=true;
        }

        elf +=1;

        if elf == 3 {
            let mut val = 0;
            for (a,b,c) in izip!(elfs_arr[0], elfs_arr[1],elfs_arr[2]) {
                val += 1;
                if a && b && c { 
                    total += val;
                    elf = 0;
                    break;
                }
            }
        }
    }

    println!("{}", total);
}
