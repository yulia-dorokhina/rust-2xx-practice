use std::io::{self, BufRead};

fn bon_appetit(bill: &[i32], k: usize, b: i32) {
    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k]) / 2;

    if b > anna_share {
        println!("{}", b - anna_share);
    } else {
        println!("Bon Appetit");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<usize>().unwrap(); // Changed to usize
    let k = first_multiple_input[1].trim().parse::<usize>().unwrap(); // Changed to usize

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bon_appetit(&bill, k, b);
}