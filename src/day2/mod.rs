use std::collections::HashMap;

use crate::{prelude::*, utils};

#[derive(Eq, PartialEq, Hash)]
enum Movement {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(Eq, PartialEq, Hash)]
enum Tactic {
    LOSE,
    DRAW,
    WIN
}

enum Mode {
    MOVES,
    TACTIC
}

struct Hands {
    moves: HashMap<Movement, HashMap<Tactic, Movement>>
}

impl Hands {
    fn new() -> Self {
        Self {
            moves: HashMap::from([
                (Movement::PAPER, HashMap::from([
                    (Tactic::WIN, Movement::ROCK),
                    (Tactic::DRAW, Movement::PAPER),
                    (Tactic::LOSE, Movement::SCISSORS)
                ])),
                (Movement::ROCK, HashMap::from([
                    (Tactic::WIN, Movement::SCISSORS),
                    (Tactic::DRAW, Movement::ROCK),
                    (Tactic::LOSE, Movement::PAPER)
                ])),
                (Movement::SCISSORS, HashMap::from([
                    (Tactic::WIN, Movement::PAPER),
                    (Tactic::DRAW, Movement::SCISSORS),
                    (Tactic::LOSE, Movement::ROCK)
                ])),
            ])
        }
    }
}

#[test]
fn test_a() {
    let result = solve("day2/input_test.txt", Mode::MOVES);
    assert_eq!(result, 15);
}

#[test]
fn test_b() {
    let result = solve("day2/input_test.txt", Mode::TACTIC);
    assert_eq!(result, 12);
}

fn solve(filename: &str, mode: Mode) -> u32 {
    let lines = utils::read_file_to_vec::<String>(filename);
    let mut total_score = 0;
    let hands = Hands::new();
    lines.into_iter().for_each(|line| {
        let moves = line.as_ref().expect("Error unwrapping line ref").split(" ");
        let mut moves_truncated: Vec<&str> = moves.collect();
        moves_truncated.truncate(2);
        match mode {
            Mode::MOVES => {
                total_score += get_score(
                    &get_movement(moves_truncated[0]).unwrap(),
                    &get_movement(moves_truncated[1]).unwrap()
                );
            },
            Mode::TACTIC => {
                let tactic = get_tactic(moves_truncated[1]).unwrap();
                let movement = get_movement(moves_truncated[0]).unwrap();
                total_score += get_score(
                    &movement,
                    hands.moves.get(&movement).expect("Movement not found").get(&tactic).expect("Tactic not found")
                );
            }
        }
    });
    total_score
}

pub struct Day2;

impl Problem<u32> for Day2 {
    fn get_day(&self) -> Result<&'static str, &'static str> {
        Ok("day2")
    }

    fn get_result_a(&self) -> Result<u32, &'static str> {
         Ok(solve("day2/input_a.txt", Mode::MOVES))
    }
    
    fn get_result_b(&self) -> Result<u32, &'static str> {
        Ok(solve("day2/input_a.txt", Mode::TACTIC))
    }
}

fn get_movement(char: &str) -> Result<Movement, &str> {
    match char {
        "A" | "X" => Ok(Movement::ROCK),
        "B" | "Y" => Ok(Movement::PAPER),
        "C" | "Z" => Ok(Movement::SCISSORS),
        &_ => Err("Movement not supported")
    }
}

fn get_tactic(char: &str) -> Result<Tactic, &str> {
    match char { // están al revés de la explicación de la web ya que lo miro desde el punto de vista de la mano izq
        "Z" => Ok(Tactic::LOSE),
        "Y" => Ok(Tactic::DRAW),
        "X" => Ok(Tactic::WIN),
        &_ => Err("Tactic not supported")
    }
}

fn get_score(hand_a: &Movement, hand_b: &Movement) -> u32 {
    // esto lo podría haber hecho algo más genérico con la impl de hands que he hecho arriba
    // pero la hice para la parte b, y no he rehecho esto ya
    match (hand_a, hand_b) {
        (a, b) if *a == *b => 3 + get_score_from_movement(b), // empate
        (Movement::ROCK, Movement::PAPER) => 6 + get_score_from_movement(&Movement::PAPER), // ganas y 2 puntos por papel
        (Movement::ROCK, Movement::SCISSORS) => 0 + get_score_from_movement(&Movement::SCISSORS), // pierdes y 3 puntos por tijeras
        (Movement::PAPER, Movement::ROCK) => 0 + get_score_from_movement(&Movement::ROCK), // pierdes y 1 punto por piedra
        (Movement::PAPER, Movement::SCISSORS) => 6 + get_score_from_movement(&Movement::SCISSORS), // ganas y 3 puntos por tijeras
        (Movement::SCISSORS, Movement::PAPER) => 0 + get_score_from_movement(&Movement::PAPER), //pierdes y 2 puntos por papel
        (Movement::SCISSORS, Movement::ROCK) => 6 + get_score_from_movement(&Movement::ROCK), // ganas y 1 punto por piedra
        (_, _) => 0 // no implementado
    }
}

fn get_score_from_movement(movement: &Movement) -> u32 {
    match movement {
        Movement::ROCK => 1,
        Movement::PAPER => 2,
        Movement::SCISSORS => 3
    }
}