use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1() {
	let filename="day1/input.txt";
	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);

        let mut number_list : Vec<isize>= Vec::new();
        let mut tmp :isize= 0;

	for (_index, line) in reader.lines().enumerate() {
		let line = line.unwrap();
                
                
                if line.is_empty()  {
                    number_list.push(tmp);
                    tmp = 0;
                } else {
                    tmp += line.parse::<isize>().unwrap();
                }
	}

        number_list.sort();
        println!("{:?}", number_list);
        
        let last_three_sum :isize= number_list.iter().rev().take(3).sum();
        println!("{}",last_three_sum);
}
