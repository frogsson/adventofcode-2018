use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Info {
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    sorter: i32,
    action: String,
}

fn main() {
    let content = fs::read_to_string("input").expect("err");

    let mut notes = Vec::new();

    for event in content.lines() {
        let mut t: Vec<&str> = event.split("] ").collect();
        let datetime: Vec<&str> = t[0].split(|c| c == ' ' || c == '-' || c == ':' || c == '[').collect();
        let action: String =  event.split("] ").collect::<Vec<_>>()[1].to_string();
        let mut sorter = String::new();
        for i in 2..6 {
            sorter.push_str(datetime[i]);
        }

        let xoxo = Info {
            month: datetime[2].parse().unwrap(),
            day: datetime[3].parse().unwrap(),
            hour: datetime[4].parse().unwrap(),
            minute: datetime[5].parse().unwrap(),
            sorter: sorter.parse().unwrap(),
            action: action,
        };

        notes.insert(0, xoxo);
    }

    notes.sort_by(|a, b| a.sorter.cmp(&b.sorter));

    // Info { month: 11, day: 1, hour: 0, minute: 0, sorter: 11010000, action: "Guard #10 begins shift" }
    // Info { month: 11, day: 1, hour: 0, minute: 5, sorter: 11010005, action: "falls asleep" }
    // Info { month: 11, day: 1, hour: 0, minute: 25, sorter: 11010025, action: "wakes up" }

    let mut guards = HashMap::new();
    let mut guardid: &str = "";
    let mut sleep_minute: i8 = 0;

    for note in &notes {
        // println!("{:?}", note);
        let action_parse: Vec<&str> = note.action.split(" ").collect();

        if action_parse[0] == "Guard" {
            guardid = action_parse[1];
            if ! guards.contains_key(guardid) {
                let mut h = HashMap::new();
                for num in 0..60 {
                    h.insert(num, 0);
                }
                guards.insert(guardid, h);
            }
        } else if action_parse[0] == "falls" {
            sleep_minute = note.minute;
        } else if action_parse[0] == "wakes" {
            for n in sleep_minute..note.minute {
                let mut up = guards.get_mut(guardid).unwrap();
                *up.entry(n).or_insert(0) += 1;
            }
        }
    }

    let mut sleepiest_guard = 0;
    let mut sleepiest_minute = 0;
    let mut minutes_asleep = 0;

    let mut sleepiest_guard_p2 = 0;
    let mut sleepiest_minute_p2 = 0;
    let mut tt = 0;

    for (gid, minute_table) in &guards {
        let mut m = 0;

        for (_, v) in minute_table {
            m += v;
        }

        if m > minutes_asleep {
            minutes_asleep = m;
            sleepiest_guard = gid.trim_start_matches("#").to_string().parse::<i32>().unwrap();

            let mut t = 0;
            for (k, v) in minute_table {
                if v > &t {
                    t = *v;
                    sleepiest_minute = *k;
                }
            }
            
        }
        // part 2
        for (k, v) in minute_table {
            if v > &tt {
                tt = *v;
                sleepiest_minute_p2 = *k;
                sleepiest_guard_p2 = gid.trim_start_matches("#").to_string().parse::<i32>().unwrap();
            }
        }
    }

    println!("part 1: {}", sleepiest_guard * sleepiest_minute as i32);
    println!("part 2: {}", sleepiest_guard_p2 * sleepiest_minute_p2 as i32);
}
