use core::panic;

use crate::{prelude::*, utils};

pub struct Day3;

const MIN_DIFF: u32 = 96;
const MAY_DIFF: u32 = 38;

#[test]
fn test_a() {
    let problem = Day3;
    let result = problem.solve("day3/input_test.txt").expect("Error getting result for test_a");
    assert_eq!(result, 157);
}

#[test]
fn test_b() {
    let result = Day3::solve_b("day3/input_test.txt").expect("Error getting result for test_b");
    assert_eq!(result, 70);
}

impl Day3 {
    fn solve_b(filename: &str) -> Result<u32, &'static str> {
        let lines: Vec<Option<String>> = utils::read_file_to_vec(filename);
        let mut buffer: Option<String> = None;
        let mut coincidences: Vec<char> = Vec::new();
        lines.into_iter().enumerate().map(|(i, line)| {
            match line {
                Some(content) => {
                    let mut value = 0;
                    match i % 3 {
                        0 => {
                            buffer = Some(content);
                        },
                        1 => {
                            for char_a in buffer.as_ref().expect("No buffer data").chars() {
                                for char_b in content.chars() {
                                    if char_a == char_b {
                                        coincidences.push(char_a);
                                    }
                                }
                            }
                        },
                        2 => {
                            for char_a in &coincidences {
                                for char_b in content.chars() {
                                    if char_a == &char_b {
                                        let char_as_int = *char_a as u32;
                                        value = if char_a.is_uppercase() { char_as_int - MAY_DIFF } else { char_as_int - MIN_DIFF }
                                    }
                                }
                            }
                        },
                        _ => panic!("Unexpected value"),
                    }
                    value
                },
                None => 0,
            }
        }).reduce(|a, b| a + b).ok_or("Error getting final value")
    }
}

impl Problem for Day3 {
    fn get_day(&self) -> Result<&'static str, &'static str> {
        Ok("day3")
    }

    fn get_result_a(&self) -> Result<u32, &'static str> {
        self.solve("day3/input_a.txt")
    }
    
    fn get_result_b(&self) -> Result<u32, &'static str> {
        Self::solve_b("day3/input_a.txt")
    }

    fn solve(&self, filename: &str) -> Result<u32, &'static str> {
        let lines: Vec<Option<String>> = utils::read_file_to_vec(filename);
        lines.into_iter().map(|line| {
            let value: u32 = match line {
                Some(content) => {
                    let mid_length: usize = content.chars().count() / 2;
                    let first_part: String = content.chars().take(mid_length).collect();
                    let second_part: String = content.chars().skip(mid_length).collect();
                    let mut repeated_as_u32 = 0;
                    let mut is_uppercase: Option<bool> = None;
                    for char_a in first_part.chars() {
                        for char_b in second_part.chars() {
                            if char_a == char_b {
                                repeated_as_u32 = char_a as u32;
                                is_uppercase = Some(char_a.is_uppercase());
                            }
                        }
                    };
                    match is_uppercase {
                        Some(upper) => if upper { repeated_as_u32 - MAY_DIFF } else { repeated_as_u32 - MIN_DIFF },
                        None => panic!("Error at retrieving uppercase or lowercase value")
                    }
                },
                None => 0
            };
            value
        }).reduce(|a, b| a + b).ok_or("Error getting final value")
    }
}