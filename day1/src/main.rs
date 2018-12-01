use std::fs;

fn main() {
    let content = fs::read_to_string("puzzleinput")
        .expect("error reading file");

    let mut freqvec = Vec::new();
    let mut frequencyfin: i32 = 0;
    let mut frequency: i32 = 0;
    let mut firsttwice: i32 = 0;
    let mut iteration: i32 = 1;
    let mut br = true;

    while br {
        println!("starting iteration: {}", iteration);
        for l in content.lines() {
            match l.parse::<i32>() {
                Ok(n) => {
                    // let beforechange = frequency; 
                    frequency += n;
                    if iteration == 1 {
                        frequencyfin += n;
                    }
                    // println!("Current frequency {}, change of {}, resulting {}.", beforechange, l, frequency)
                }
                Err(e) => println!("{}", e)
            }

            for value in freqvec.iter() {
                if *value == frequency && br { 
                    firsttwice = frequency; 
                    br = false;
                }
            }

            freqvec.push(frequency);
        }
        iteration += 1;
    }

    println!("challenge 1 answer: {}", frequencyfin);
    println!("challenge 2 answer: {}", firsttwice);
}
