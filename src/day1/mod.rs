use crate::{prelude::*, Problem};
use std::collections::HashMap;

#[test]
fn test_a() {
    let result = solve("day1/input_test.txt", HashMap::from([("number_results", "1")]));
    assert_eq!(result.expect("Error getting result"), 24000);
}

#[test]
fn test_b() {
    let result = solve("day1/input_test.txt", HashMap::from([("number_results", "3")]));
    assert_eq!(result.expect("Error getting result"), 45000);
}

pub struct Day1;

fn solve(filename: &str, extra_params: HashMap<&str, &str>) -> Result<u32, &'static str> {
    let lines: Vec<Option<String>> = read_file_to_vec(filename);
    let mut sum = 0;
    let mut sums_vec: Vec<u32> = Vec::with_capacity(lines.len());
    lines.into_iter()
        .for_each(|line| {
            let nums = match line {
                Some(nums) => nums.parse::<u32>().unwrap(),
                None => {
                    // esto no me termina, pero bueno.. de momento lo dejamos así
                    sums_vec.push(sum);
                    sum = 0;
                    0
                }
            };
            sum += nums;
        });
    sums_vec.sort_by(|a, b| b.cmp(a));
    sums_vec.truncate(extra_params.get("number_results").expect("Param not found").parse::<usize>().expect("Error parsing usize"));
    Ok(sums_vec.into_iter().reduce(|a, b| a + b).expect("Failed adding numbers"))
}

impl Problem<u32> for Day1 {
    fn get_day(&self) -> Result<&'static str, &'static str> {
        Ok("day1")
    }

    fn get_result_a(&self) -> Result<u32, &'static str> {
        solve("day1/input_a.txt", HashMap::from([("number_results", "1")]))
    }
    
    fn get_result_b(&self) -> Result<u32, &'static str> {
        solve("day1/input_a.txt", HashMap::from([("number_results", "3")]))
    }
}
