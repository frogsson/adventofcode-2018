use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input").expect("error reading file");
    let mut h = HashMap::new();
    let (mut two, mut three) = (0, 0);
    let mut p2 = Vec::new();

    // p1
    for line in contents.lines() {
        h.clear();
        for ch in line.chars() {
            let c = h.entry(ch).or_insert(0);
            *c += 1;
        }

        let (mut btwo, mut bthree) = (true, true); // whatever
        for val in h.values() {
            if *val == 2 && btwo {
                two += 1;
                btwo = false;
                p2.insert(0, line);
            } else if *val == 3 && bthree {
                three += 1;
                bthree = false;
                p2.insert(0, line);
            }
        }
    }

    // p2
    let mut matches = 0;
    let mut chrs = String::new();
    for line in p2.iter() {
        for lone in p2.iter() {
            let mut lmatches = 0;
            let mut lchrs = String::new();
            if line == lone { 
                break
            } else {
                let test = line.chars().zip(lone.chars());
                for (ch1, ch2) in test {
                    if ch1 == ch2 {
                        lmatches += 1;
                        lchrs.push(ch1);
                    }
                }
            }
            if lmatches > matches {
                matches = lmatches;
                chrs = lchrs;
            }
        }
    }

    println!("part 1 = {}", two * three); // p1
    println!("part 2 = {}", chrs);
}
