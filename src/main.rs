use std::{collections::HashMap, sync::Mutex};
use lazy_static::lazy_static;

mod day1;
mod day2;
mod utils;

mod prelude {
    pub use crate::utils::*;
}

lazy_static! {
    static ref PROBLEMS: Mutex<HashMap<&'static str, fn()>> = {
        let hm = HashMap::new();
        Mutex::new(hm)
    };
}

fn main() {
    utils::add_problem("1-a", day1::problem_a);
    utils::add_problem("1-b", day1::problem_b);
    utils::add_problem("2-a", day2::problem_a);
    utils::add_problem("2-b", day2::problem_b);

    utils::exec_problem("2-a");
    utils::exec_problem("2-b");
}
