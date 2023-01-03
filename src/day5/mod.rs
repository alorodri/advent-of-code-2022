use crate::prelude::*;

pub struct Day5;

#[test]
fn test_a() {
    let problem = Day5;
    let result = problem.solve("day5/input_test.txt", ProblemChoice::A).expect("Error getting test_a result");
    assert_eq!(result, "CMZ");
}

impl Problem<&'static str> for Day5 {
    fn get_day(&self) -> Result<&'static str, &'static str> {
        Ok("day5")
    }

    fn solve(&self, filename: &str, _problem: ProblemChoice) -> Result<&'static str, &'static str> {
        let lines: Vec<Option<String>> = read_file_to_vec(filename);
        panic!();
    }
}