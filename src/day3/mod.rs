use core::panic;

use crate::{prelude::*, utils};

pub struct Day3;

const MIN_DIFF: u32 = 96;
const MAY_DIFF: u32 = 38;

#[test]
fn test_a() {
    let result = Day3::solve("day3/input_test.txt").expect("Error getting result for test_a");
    assert_eq!(result, 157);
}

impl Problem for Day3 {
    fn get_day() -> Result<&'static str, &'static str> {
        Ok("day3")
    }

    fn get_result_a() -> Result<u32, &'static str> {
        Self::solve("day3/input_a.txt")
    }

    fn solve(filename: &str) -> Result<u32, &'static str> {
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