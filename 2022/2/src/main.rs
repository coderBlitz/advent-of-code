use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Move {
	Rock = 1,
	Paper = 2,
	Scissor = 3,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
	Lose = 0,
	Draw = 3,
	Win = 6,
}

impl PartialOrd for Move {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self < other {
			Some(Ordering::Less)
		} else if self > other {
			Some(Ordering::Greater)
		} else {
			Some(Ordering::Equal)
		}
	}
	fn lt(&self, other: &Self) -> bool {
		match self {
			Move::Rock => *other == Move::Paper,
			Move::Paper => *other == Move::Scissor,
			Move::Scissor => *other == Move::Rock,
		}
	}
	fn le(&self, other: &Self) -> bool {
		!(self.gt(other))
	}
	fn gt(&self, other: &Self) -> bool {
		match self {
			Move::Rock => *other == Move::Scissor,
			Move::Paper => *other == Move::Rock,
			Move::Scissor => *other == Move::Paper,
		}
	}
	fn ge(&self, other: &Self) -> bool {
		!(self.lt(other))
	}
}

impl Outcome {
	pub fn move_given_opponent(&self, opponent_move: &Move) -> Move {
		match self {
			Outcome::Lose => {
				match opponent_move {
					Move::Rock => Move::Scissor,
					Move::Paper => Move::Rock,
					Move::Scissor => Move::Paper,
				}
			},
			Outcome::Draw => *opponent_move,
			Outcome::Win => {
				match opponent_move {
					Move::Rock => Move::Paper,
					Move::Paper => Move::Scissor,
					Move::Scissor => Move::Rock,
				}
			},
		}
	}
}

fn main() {
	/* Get reader for input file
	*/
	let argv: Vec<String> = env::args().collect();

	let filename;
	if argv.len() > 1 {
		filename = argv[1].clone();
	} else {
		filename = String::from("./input");
	}

	let file = File::open(filename).expect("Could not open file");
	let mut reader = BufReader::new(file);

	/* Do challenge
	*/
	let mut line = String::new();
	let mut total_points_1 = 0;
	let mut total_points_2 = 0;
	let mut _lines = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		// Line reader EOF condition
		if length == 0 {
			break;
		}

		// Part 1 things
		let elf_move = match &line[0..1] {
			"A" => Move::Rock,
			"B" => Move::Paper,
			"C" => Move::Scissor,
			_ => panic!("Unknown move on line {_lines}!"),
		};
		let my_move = match &line[2..3] {
			"X" => Move::Rock,
			"Y" => Move::Paper,
			"Z" => Move::Scissor,
			_ => panic!("Unknown move on line {_lines}!"),
		};
		let game_result = match my_move.partial_cmp(&elf_move).unwrap() {
			Ordering::Less => Outcome::Lose,
			Ordering::Equal => Outcome::Draw,
			Ordering::Greater => Outcome::Win,
		};

		// Part 2 things
		let intended_result = match &line[2..3] {
			"X" => Outcome::Lose,
			"Y" => Outcome::Draw,
			"Z" => Outcome::Win,
			_ => panic!("Unknown move on line {_lines}!"),
		};
		let planned_move = intended_result.move_given_opponent(&elf_move);

		total_points_1 += my_move as i32 + game_result as i32;
		total_points_2 += planned_move as i32 + intended_result as i32;

		_lines += 1;
		line.clear();
	}

	println!("You earned {total_points_1} points!");
	println!("You are supposed to earn {total_points_2} points!");
}

/** Tests section
**/
#[test]
fn rock_relations() {
	assert!(Move::Rock == Move::Rock);
	assert!(Move::Rock < Move::Paper);
	assert!(Move::Rock > Move::Scissor);
}
#[test]
fn paper_relations() {
	assert!(Move::Paper == Move::Paper);
	assert!(Move::Paper < Move::Scissor);
	assert!(Move::Paper > Move::Rock);
}
#[test]
fn scissor_relations() {
	assert!(Move::Scissor == Move::Scissor);
	assert!(Move::Scissor < Move::Rock);
	assert!(Move::Scissor > Move::Paper);
}
