use std::io::{self, BufRead};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = (grade / 5 + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        }
    }).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let grades_count: usize = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count);

    for _ in 0..grades_count {
        let grades_item: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    for (i, &grade) in result.iter().enumerate() {
        print!("{}", grade);
        if i != result.len() - 1 {
            print!(" ");
        }
    }
    println!();
}