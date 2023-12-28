// ADVENT OF CODE 2017 - DAY 02

use std::fs::File;
use std::io::{self, BufRead};

fn main() {

    /*
    ** PART 1
    */
    let file = File::open("../Inputs/day02input.txt").expect("Error");
    let reader = io::BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Error");
        let digits: Vec<u32> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();

        sum += digits.iter().max().zip(digits.iter().min()).map(|(max, min)| max - min).unwrap_or(0);
    }
    println!("{}", sum);

    /*
    ** PART 2
    */
    let file = File::open("../Inputs/day02input.txt").expect("Error");
    let reader = io::BufReader::new(file);

    sum = 0;

    for line in reader.lines() {
        let line = line.expect("Error");
        let digits: Vec<u32> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();

        for (index, &num1) in digits.iter().enumerate() {
            for &num2 in &digits[index + 1..] {
                if num1 % num2 == 0 { sum += num1 / num2 }
                if num2 % num1 == 0 { sum += num2 / num1 }
            }
        }
    }
    println!("{}", sum);
}