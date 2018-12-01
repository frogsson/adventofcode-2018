use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("puzzleinput2")
        .expect("error reading file");

    let mut hashfreq = HashSet::new();
    let mut frequency: i32 = 0;
    let mut iteration: bool = true;
    let mut notreachtwice: bool = true;

    let mut ch1: i32 = 0;

    while notreachtwice {
        for l in content.lines() {
            frequency += l.parse::<i32>().unwrap();

            notreachtwice = hashfreq.insert(frequency);
            if ! notreachtwice { break }
        }
        if iteration { ch1 = frequency; iteration = false }
    }
    let ch2 = frequency;

    println!("part 1 answer: {}", ch1);
    println!("part 2 answer: {}", ch2);
}
