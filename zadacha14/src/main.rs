use std::io::{self, BufRead};

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut max_record = scores[0];
    let mut min_record = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;

    for &score in &scores[1..] {
        if score > max_record {
            max_record = score;
            max_count += 1;
        } else if score < min_record {
            min_record = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n: usize = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    println!("{} {}", result[0], result[1]);
}