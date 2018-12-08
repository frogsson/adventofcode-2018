use std::fs;
use std::collections::HashSet;

fn poly_cleanse(mut con: String) -> String {
    'makin_sure: loop {
        let mut br = true;
        let mut num = 0;
        while num < con.len() {
            let two_chars = con.get(num..num+2).unwrap_or_default().to_string();

            if two_chars.len() > 1 { // check if not EOL
                let chars: Vec<char> = two_chars.chars().collect();

                if chars[0].to_lowercase().to_string() == chars[1].to_lowercase().to_string()
                && chars[0].is_lowercase() != chars[1].is_lowercase() {
                    con.remove(num);
                    con.remove(num);
                    br = false;
                    if num >= 2 {
                        num -= 2;
                    }
                }
            }
            num += 1;
        }
        if br {
            break 'makin_sure;
        }
    }
    con
}

fn main() {
    let mut contents = fs::read_to_string("input").expect("err").replace("\n", "");

    contents = poly_cleanse(contents);

    let p1 = contents.len();
    let mut parsed_chars: HashSet<char> = HashSet::new();
    let mut p2 = p1;

    for poly in contents.chars() {
        let ch: char = poly.to_lowercase().to_string().parse::<char>().unwrap();

        if ! parsed_chars.contains(&ch) {
            parsed_chars.insert(ch);
            let upper_case = ch.to_uppercase().to_string().parse::<char>().unwrap();

            let mut new_contents = contents.replace(ch, "").replace(upper_case, "");
            new_contents = poly_cleanse(new_contents);

            let new_contents_len = new_contents.len();
            if new_contents_len < p2 {
                p2 = new_contents_len;
            }
        }
    }

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
