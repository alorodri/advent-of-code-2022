use crate::prelude::*;
use std::io::BufRead;

#[test]
fn test_a() {
    let result = solve_general("day1/input_test.txt", 1);
    assert_eq!(result, 24000);
}

#[test]
fn test_b() {
    let result = solve_general("day1/input_test.txt", 3);
    assert_eq!(result, 45000);
}

fn solve_general(filename: &str, number_results: usize) -> u32 {
    let br = open_file(filename);
    let mut sum = 0;
    let lines: Vec<_> = br.lines().collect();
    let mut sums_vec: Vec<u32> = Vec::with_capacity(lines.len());
    lines.into_iter()
        .for_each(|line| {
            let num_chars = line.as_ref().unwrap().chars().count();
            if num_chars > 0 {
                sum += line.unwrap().parse::<u32>().unwrap();
            } else {
                sums_vec.push(sum);
                sum = 0;
            }
        });
    sums_vec.sort_by(|a, b| b.cmp(a));
    sums_vec.truncate(number_results);
    sums_vec.into_iter().reduce(|a, b| a + b).expect("Failed adding numbers")
}

pub fn problem_a() {
    let result = solve_general("day1/input_a.txt", 1);
    write_result("day1/result_a.txt", &result.to_string()).expect("Error writting result");
}

pub fn problem_b() {
    let result = solve_general("day1/input_a.txt", 3);
    write_result("day1/result_b.txt", &result.to_string()).expect("Error writting result");
}