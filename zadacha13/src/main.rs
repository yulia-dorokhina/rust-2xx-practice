use std::io::{self, BufRead};

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i32, y: i32) -> i32 {
    (x * y) / gcd(x, y)
}

fn get_total_lx(a: &[i32], b: &[i32]) -> i32 {
    let mut lcm_a = a[0];
    for &num in &a[1..] {
        lcm_a = lcm(lcm_a, num);
    }

    let mut gcd_b = b[0];
    for &num in &b[1..] {
        gcd_b = gcd(gcd_b, num);
    }

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_line = stdin_iterator.next().unwrap().unwrap();
    let sizes: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let n = sizes[0] as usize;
    let m = sizes[1] as usize;

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let total = get_total_lx(&a, &b);
    println!("{}", total);
}