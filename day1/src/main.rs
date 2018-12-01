use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("puzzleinput")
        .expect("error reading file");

    let mut hashfreq = HashSet::new();
    let mut frequency: i32 = 0;
    let mut ch1hasvalue: bool = false; // a bit ugly but works
    let mut freqnotfound: bool = true;

    let mut ch1: i32 = 0;
    let mut ch2: i32 = 0;

    while freqnotfound {
        for l in content.lines() {
            frequency += l.parse::<i32>().unwrap(); // uneccssary parsing for every iteration

            freqnotfound = hashfreq.insert(frequency);
            if ! freqnotfound { 
                ch2 = frequency; 
                if ch1hasvalue { break }
            }
        }
        if ! ch1hasvalue { ch1 = frequency; ch1hasvalue = true }
    }

    println!("part 1 answer: {}", ch1);
    println!("part 2 answer: {}", ch2);
}
