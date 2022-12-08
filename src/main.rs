mod day1;
mod day2;
mod day3;

use std::time::Instant;


fn main() {
    
    println!("DAY 1");
    //day one solution
    let start = Instant::now();
    day1::day1();
    println!("execution took: {}ms",start.elapsed().as_micros());

    println!("\nDAY 2");
    //day two solution
    let start = Instant::now();
    day2::day2_1();
    println!("execution took: {}ms",start.elapsed().as_micros());
    let start = Instant::now();
    day2::day2_2();
    println!("execution took: {}ms",start.elapsed().as_micros());

    println!("\nDAY 3");
    //day three solution
    let start = Instant::now();
    day3::day3_1();
    println!("execution took: {}ms",start.elapsed().as_micros());
    let start = Instant::now();
    day3::day3_2();
    println!("execution took: {}ms",start.elapsed().as_micros());
}
