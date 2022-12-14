pub mod day1;
pub mod day2;
pub mod utils;

fn main() {
    utils::create_all_problems();
    // ejecutar un problema concreto
    // utils::exec_problem("2-b");
    utils::exec_all_problems();
}

pub trait Problem {
    fn get_file_contents(filename: &str) -> Vec<Option<String>> {
        utils::read_file_to_vec(filename)
    }

    fn solve(_filename: &str) -> Result<u32, &'static str> {
        Err("Not implemented")
    }

    fn get_result_a() -> Result<u32, &'static str> {
        Err("Not implemented")
    }

    fn get_result_b() -> Result<u32, &'static str> {
        Err("Not implemented")
    }

    fn get_day() -> Result<&'static str, &'static str> {
        Err("Day not implemented")
    }

    fn problem_a() {
        let result = Self::get_result_a();
        let day = Self::get_day().unwrap();
        utils::write_result(format!("{}/result_a.txt", day).as_str(), &result.expect("Error getting result").to_string()).expect("Error writting result");
    }

    fn problem_b() {
        let result = Self::get_result_b();
        let day = Self::get_day().unwrap();
        utils::write_result(format!("{}/result_b.txt", day).as_str(), &result.expect("Error getting result").to_string()).expect("Error writting result");
    }
}

mod prelude {
    pub use crate::utils::*;
    pub use crate::day1::*;
    pub use crate::day2::*;
    pub use crate::Problem;
}