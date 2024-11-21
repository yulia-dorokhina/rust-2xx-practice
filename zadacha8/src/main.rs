use std::io::{self, BufRead};

fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap();

    candles.iter().filter(|&&height| height == max_height).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _candles_count: i32 = stdin_iterator.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let candles: Vec<i32> = stdin_iterator.next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);
    println!("{}", result);
}
