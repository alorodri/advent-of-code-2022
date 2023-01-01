use crate::{prelude::*, utils};

pub struct Day4;

#[test]
fn test_a() {
    let problem = Day4;
    let result = problem.solve("day4/input_test.txt", ProblemChoice::A).expect("Error on test_a result");
    assert_eq!(result, 2);
}

#[test]
fn test_b() {
    let problem = Day4;
    let result = problem.solve("day4/input_test.txt", ProblemChoice::B).expect("Error on test_b result");
    assert_eq!(result, 4);
}

impl Problem for Day4 {
    fn get_day(&self) -> Result<&'static str, &'static str> {
        Ok("day4")
    }

    fn get_result_a(&self) -> Result<u32, &'static str> {
        self.solve("day4/input_a.txt", ProblemChoice::A) 
    }

    fn get_result_b(&self) -> Result<u32, &'static str> {
        self.solve("day4/input_a.txt", ProblemChoice::B)
    }

    fn solve(&self, filename: &str, problem: ProblemChoice) -> Result<u32, &'static str> {
        let lines: Vec<Option<String>> = utils::read_file_to_vec(filename);
        let mut result: u32 = 0;
        lines.iter().for_each(|line| {
            match line {
                Some(content) => {
                    let elves = content.split(",");
                    let mut buffer: Option<&str> = None;
                    for elf in elves {
                        if buffer.is_none() {
                            buffer = Some(&elf);
                        } else {
                            #[allow(unused_assignments)]
                            let mut overlaping = None;
                            if problem == ProblemChoice::A {
                                overlaping = Some(overlaps(&elf, buffer.unwrap()));
                            } else {
                                overlaping = Some(partial_overlaps(&elf, buffer.unwrap()));
                            }
                            if overlaping.unwrap() {
                                result += 1;
                                #[cfg(test)]
                                println!("{} overlaps with {}", &elf, buffer.unwrap());
                            }
                        }
                    }
                },
                None => {}
            }
        });
        Ok(result)
    }
}

fn overlaps(range1: &str, range2: &str) -> bool {
    let r1 = range1.split("-").collect::<Vec<&str>>();
    let r1_start = convert_to_u32(r1[0]);
    let r1_end = convert_to_u32(r1[1]);

    let r2 = range2.split("-").collect::<Vec<&str>>();
    let r2_start = convert_to_u32(r2[0]);
    let r2_end = convert_to_u32(r2[1]);

    (r2_start >= r1_start && r2_start <= r1_end && r2_end >= r1_start && r2_end <= r1_end)
        || (r1_start >= r2_start && r1_start <= r2_end && r1_end >= r2_start && r1_end <= r2_end)
}

fn partial_overlaps(range1: &str, range2: &str) -> bool {
    let r1 = range1.split("-").collect::<Vec<&str>>();
    let r1_start = convert_to_u32(r1[0]);
    let r1_end = convert_to_u32(r1[1]);

    let r2 = range2.split("-").collect::<Vec<&str>>();
    let r2_start = convert_to_u32(r2[0]);
    let r2_end = convert_to_u32(r2[1]);

    (r1_start >= r2_start && r1_start <= r2_end) || (r1_end >= r2_start && r1_end <= r2_end)
        || (r2_start >= r1_start && r2_start <= r1_end) || (r2_end >= r1_start && r2_end <= r1_end)
}

fn convert_to_u32(str: &str) -> u32 {
    str.parse::<u32>().expect("Error parsing str to u32")
}