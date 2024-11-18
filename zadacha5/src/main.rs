use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64;
    let (mut positive_count, mut negative_count, mut zero_count) = (0, 0, 0);

    for &value in arr {
        if value > 0 {
            positive_count += 1;
        } else if value < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let positive_ratio = positive_count as f64 / n;
    let negative_ratio = negative_count as f64 / n;
    let zero_ratio = zero_count as f64 / n;

    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}