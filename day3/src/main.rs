use std::fs;

fn main() {
    let mut fabx = vec![vec![0;1001]; 1001];
    let elves = fs::read_to_string("input").expect("error reading file");

    for elf in elves.lines() {
        let s: Vec<&str> = elf.split(" ").collect();
        let mut c = String::from(s[2]);
        c.pop();
        let c: Vec<&str> = c.split(",").collect();

        let startx = c[1].parse::<i16>().unwrap();
        let starty = c[0].parse::<i16>().unwrap();

        let c = String::from(s[3]);
        let c: Vec<&str> = c.split("x").collect();

        let mut claimx = c[1].parse::<i16>().unwrap();
        let mut claimy = c[0].parse::<i16>().unwrap();

        for x in startx..startx+claimx {
            for y in starty..starty+claimy {
                fabx[x as usize][y as usize] += 1;
            }
        }
    }
    let mut count = 0;
    for x in fabx.iter() {
        for y in x.iter() {
            if y >= &2 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
