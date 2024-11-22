use std::collections::HashMap;
use std::io::{self, BufRead};

fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut color_count = HashMap::new();

    for &color in ar {
        *color_count.entry(color).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in color_count.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    println!("{}", result);
}