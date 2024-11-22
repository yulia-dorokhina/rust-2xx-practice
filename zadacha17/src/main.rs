use std::io::{self, BufRead};
use std::collections::HashMap;

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut bird_count = HashMap::new();

    for &bird in arr {
        *bird_count.entry(bird).or_insert(0) += 1;
    }

    let mut max_frequency = 0;
    let mut bird_id_with_max_frequency = i32::MAX;

    for (&bird_id, &count) in &bird_count {
        if count > max_frequency {
            max_frequency = count;
            bird_id_with_max_frequency = bird_id;
        } else if count == max_frequency {
            if bird_id < bird_id_with_max_frequency {
                bird_id_with_max_frequency = bird_id;
            }
        }
    }

    bird_id_with_max_frequency
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    println!("{}", result);
}