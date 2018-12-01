use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("puzzleinput")
        .expect("error reading file");

    let mut hashfreq = HashSet::new();
    let mut frequency: i32 = 0;
    let mut iteration: i32 = 0;
    let mut br = true;

    let mut ch1: i32 = 0;

    while br {
        for l in content.lines() {
            let adjustment: i32 = l.parse().unwrap();
            frequency += adjustment;

            br = hashfreq.insert(frequency);
            if ! br { break }
        }
        if iteration == 0 { ch1 = frequency }
        iteration += 1;
    }
    let ch2 = frequency;

    println!("challenge 1 answer: {}", ch1);
    println!("challenge 2 answer: {}", ch2);
}
