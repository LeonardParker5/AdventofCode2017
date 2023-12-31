// ADVENT OF CODE 2017 - DAY 04

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn has_duplicates<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let set: HashSet<_> = vec.iter().collect();
    set.len() != vec.len()
}

fn main() {

    /*
    ** PART 1
    */
    let file = File::open("../Inputs/day04input.txt").expect("Error");
    let reader = io::BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Error");
        let passphrase: Vec<&str> = line.split_whitespace().collect();
        if !has_duplicates(&passphrase) { sum += 1 }
    }
    println!("{}", sum);

    /*
    ** PART 2
    */
    let file = File::open("../Inputs/day04input.txt").expect("Error");
    let reader = io::BufReader::new(file);
    sum = 0;

    for line in reader.lines() {
        let line = line.expect("Error");
        let mut passphrase: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        passphrase.iter_mut().for_each(|s| {
            let mut chars: Vec<_> = s.chars().collect();
            chars.sort();
            *s = chars.into_iter().collect();
        });

        if !has_duplicates(&passphrase) { sum += 1 }  
    }
    println!("{}", sum);
}