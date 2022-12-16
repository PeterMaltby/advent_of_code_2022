mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::time::Instant;

fn main() {
    println!("DAY 1");
    //day one solution
    let start = Instant::now();
    day1::day1();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 2");
    //day two solution
    let start = Instant::now();
    day2::day2_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day2::day2_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 3");
    //day three solution
    let start = Instant::now();
    day3::day3_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day3::day3_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 4");
    //day four solution
    let start = Instant::now();
    day4::day4_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day4::day4_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 5");
    //day five solution
    let start = Instant::now();
    day5::day5_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day5::day5_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 6");
    //day six solution
    let start = Instant::now();
    day6::day6_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day6::day6_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 7");
    //day seven solution
    let start = Instant::now();
    day7::day7_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day7::day7_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 8");
    //day eight solution
    let start = Instant::now();
    day8::day8_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day8::day8_2();
    println!("execution took: {}ms", start.elapsed().as_micros());

    println!("\nDAY 8");
    //day nine solution
    let start = Instant::now();
    day9::day9_1();
    println!("execution took: {}ms", start.elapsed().as_micros());
    let start = Instant::now();
    day9::day9_2();
    println!("execution took: {}ms", start.elapsed().as_micros());
}
