use std::fs::File;
use std::io::{BufReader, Write, BufRead};
use std::str::FromStr;

use crate::PROBLEMS;

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

pub fn exec_problem(name: &str) {
    PROBLEMS.lock().unwrap().get(name).expect("Error retrieving problem from hashmap")();
}