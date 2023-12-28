// ADVENT OF CODE 2017 - DAY 01

use std::fs;

fn main() {

    // PART 1
    let input = fs::read_to_string("../Inputs/day01input.txt").expect("Error");
    let list: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut sum: u32 = 0;
    
    for window in list.windows(2) {
        if window[0] == window[1] { sum += window[0] }
    }
    if list[list.len() - 1] == list[0] { sum += list[0] }

    println!("{}", sum);

    // PART 2
    sum = 0;
    let half: u32 = (list.len() / 2).try_into().unwrap();
    let length: u32 = (list.len()).try_into().unwrap();
    
    for i in 0..length {
        let jump;
        if i + (half) > (length - 1) {
            jump = (i + (half)) - (length);
        } else {
            jump = i + (half)
        }
        if list[i as usize] == list[jump as usize] { sum += list[i as usize] }
    }

    println!("{}", sum);
}