use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Write, BufRead};
use std::str::FromStr;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::prelude::*;

macro_rules! cp {
    ($($v:expr),*) => {
        $({
            let problem = $v;
            problem.problem_a();
            problem.problem_b();
        })*
    };
}

pub fn execute_all_problems() {
    cp!(
        Day1,
        Day2,
        Day3,
        Day4,
        Day5
    );
}

lazy_static! {
    static ref PROBLEMS: Mutex<HashMap<&'static str, ()>> = {
        let hm = HashMap::new();
        Mutex::new(hm)
    };
}

pub fn open_file(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).expect("Error reading test file"))
}

pub fn read_file_to_vec<T: FromStr>(filename: &str) -> Vec<Option<T>> {
    let mut results: Vec<Option<T>> = Vec::new();
    open_file(filename).lines().for_each(|line| {
        match line.as_ref().expect("Failing reading file").parse::<T>() {
            Ok(content) => {
                match line.expect("Error on line ref").chars().count() {
                    0 => results.push(None),
                    _ => results.push(Some(content))
                }
            },
            Err(_) => results.push(None)
        }
    });
    results
}

#[test]
fn test_get_characters_as() {
    let string: String = get_characters_as("hello world", 2, 3);
    assert_eq!(string, "llo");
    let numbers: u32 = get_characters_as("123456789", 2, 3);
    assert_eq!(numbers, 345);
}

pub fn get_characters_as<T>(text: &'static str, init: usize, n: usize) -> T
    where T: FromStr, <T as FromStr>::Err: Debug
{
    let result: String = text.chars().skip(init).take(n).collect();
    result.parse().expect("Error parsing characters to T")
}

pub fn write_result(filename: &str, content: &str) -> std::io::Result<File> {
    let mut f = File::create(filename)?;
    f.write_all(content.as_bytes()).expect("Failed writting to a file");
    Ok(f)
}

pub fn add_problem(name: &'static str, result: ()) {
    PROBLEMS.lock().unwrap().insert(name, result);
}

#[allow(dead_code)]
pub fn exec_problem(name: &str) {
    PROBLEMS.lock().unwrap().get(name).expect("Error retrieving problem from hashmap");
}