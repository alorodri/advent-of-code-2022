pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod utils;

fn main() {
    utils::execute_all_problems();
}

#[derive(PartialEq)]
pub enum ProblemChoice {
    A, B
}

pub trait Problem<T> 
    where T: std::fmt::Display
{
    fn get_file_contents(filename: &str) -> Vec<Option<String>> {
        utils::read_file_to_vec(filename)
    }

    fn solve(&self, _filename: &str, _problem: ProblemChoice) -> Result<T, &'static str>
    {
        Err("Not implemented")
    }

    fn get_result_a(&self) -> Result<T, &'static str> {
        println!("[WARNING] Result A for {} is not implemented", self.get_day().unwrap());
        Err("Not implemented")
    }

    fn get_result_b(&self) -> Result<T, &'static str> {
        println!("[WARNING] Result B for {} is not implemented", self.get_day().unwrap());
        Err("Not implemented")
    }

    fn get_day(&self) -> Result<&'static str, &'static str> {
        Err("Day not implemented")
    }

    fn problem_a(&self) {
        let result = self.get_result_a();
        let day = self.get_day().unwrap();
        utils::write_result(format!("{}/result_a.txt", day).as_str(), &result.expect("Error getting result").to_string()).expect("Error writting result");
    }

    fn problem_b(&self) {
        let result = self.get_result_b();
        let day = self.get_day().unwrap();
        utils::write_result(format!("{}/result_b.txt", day).as_str(), &result.expect("Error getting result").to_string()).expect("Error writting result");
    }
}

mod prelude {
    pub use crate::utils::*;
    pub use crate::day1::*;
    pub use crate::day2::*;
    pub use crate::day3::*;
    pub use crate::day4::*;
    pub use crate::day5::*;
    pub use crate::Problem;
    pub use crate::ProblemChoice;
}