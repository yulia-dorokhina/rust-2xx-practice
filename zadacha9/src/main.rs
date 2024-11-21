use std::io::{self, BufRead};

fn time_conversion(s: &str) -> String {
    let period = &s[s.len() - 2..];
    let time = &s[..s.len() - 2];

    let (hours, minutes_seconds) = time.split_at(2);
    let (minutes, seconds) = minutes_seconds.split_at(3);

    let mut hour: i32 = hours.parse().unwrap();

    if period == "AM" {
        if hour == 12 {
            hour = 0;
        }
    } else {
        if hour != 12 {
            hour += 12; //
        }
    }

    let hour_24 = format!("{:02}", hour);

    format!("{}{}{}", hour_24, minutes, seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    println!("{}", result);
}