use std::io::{self, BufRead};

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &apple in apples {
        let landing_position = a + apple;
        if landing_position >= s && landing_position <= t {
            apple_count += 1;
        }
    }

    for &orange in oranges {
        let landing_position = b + orange;
        if landing_position >= s && landing_position <= t {
            orange_count += 1;
        }
    }

    println!("{}", apple_count);
    println!("{}", orange_count);
}
fn read_fruit_distances(stdin_iterator: &mut std::io::Lines<io::StdinLock>, count: usize) -> Vec<i32> {
    stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .take(count)
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Зчитуємо межі будинку
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Зчитуємо позиції дерев
    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    // Зчитуємо кількість яблук і апельсинів
    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = third_multiple_input[0].trim().parse::<usize>().unwrap(); // Кількість яблук
    let n = third_multiple_input[1].trim().parse::<usize>().unwrap(); // Кількість апельсинів

    // Зчитуємо відстані падіння яблук і апельсинів
    let apples = read_fruit_distances(&mut stdin_iterator, m);
    let oranges = read_fruit_distances(&mut stdin_iterator, n);

    // Викликаємо функцію для підрахунку яблук і апельсинів
    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}