use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write, BufRead};
use std::str::FromStr;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::prelude::*;

macro_rules! cp {
    ($($k:literal, $v:expr),*) => {
        $(add_problem($k, $v);)*
    };
}

pub fn create_all_problems() {
    cp!(
        "1-a", Day1::problem_a,
        "1-b", Day1::problem_b,
        "2-a", Day2::problem_a,
        "2-b", Day2::problem_b,
        "3-a", Day3::problem_a,
        "3-b", Day3::problem_b
    );
}

lazy_static! {
    static ref PROBLEMS: Mutex<HashMap<&'static str, fn()>> = {
        let hm = HashMap::new();
        Mutex::new(hm)
    };
}

pub fn open_file(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).expect("Error reading test file"))
}

pub fn read_file_to_vec<T: FromStr>(filename: &str) -> Vec<Option<T>> {
    let lines: Vec<_> = open_file(filename).lines().collect();
    let mut results: Vec<Option<T>> = Vec::new();
    lines.into_iter().for_each(|line| {
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

pub fn write_result(filename: &str, content: &str) -> std::io::Result<File> {
    let mut f = File::create(filename)?;
    f.write_all(content.as_bytes()).expect("Failed writting to a file");
    Ok(f)
}

pub fn add_problem(name: &'static str, func: fn()) {
    PROBLEMS.lock().unwrap().insert(name, func);
}

#[allow(dead_code)]
pub fn exec_problem(name: &str) {
    PROBLEMS.lock().unwrap().get(name).expect("Error retrieving problem from hashmap")();
}

#[allow(dead_code)]
pub fn exec_all_problems() {
    for problem in PROBLEMS.lock().expect("Error getting problems ref").values() {
        problem();
    }
}