use std::fs;

fn main() {
    ch1();
}

fn ch1() {
    let content = fs::read_to_string("puzzleinput")
        .expect("error reading file");
    let mut frequency: i32 = 0;

    for l in content.lines() {
        match l.parse::<i32>() { // safe boys
            Ok(n) => {
                let beforechange = frequency; 
                frequency += n;
                println!("Current frequency {}, change of {}, resulting {}.", beforechange, l, frequency);
            }
            Err(e) => println!("{}", e)
        }
    }

    println!("{}", frequency)
}
