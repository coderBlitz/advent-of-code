use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::BitAnd;

#[derive(Default, PartialEq, Eq, Copy, Clone)]
struct AlphaSet {
	set: u64
}

impl AlphaSet {
	fn set(&mut self, c: char) {
		match c {
			'a'..='z' => {
				self.set |= 0x1 << (c as i32 - 'a' as i32);
			},
			'A'..='Z' => {
				self.set |= 0x1 << (c as i32 - 'A' as i32) + 26;
			},
			_ => panic!("Invalid character for set"),
		};
	}

	fn clear(&mut self) {
		self.set = 0;
	}

	fn is_set(&self, idx: u64) -> bool {
		if idx > 63 {
			panic!("Cannot shift more than 63!");
		}

		(self.set & (0x1 << idx)) > 0
	}
}

impl BitAnd for AlphaSet {
	type Output = AlphaSet;

	fn bitand(self, other: Self) -> Self::Output {
		AlphaSet { set: self.set & other.set }
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
	let mut _lines = 0;
	let mut total_priorities = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		// Get compartments
		let split_index = line.len() / 2;

		let comp_1 = &line[..split_index].trim();
		let comp_2 = &line[split_index..].trim();

		// Add items to sets
		let mut items_1 = AlphaSet {set: 0};
		let mut items_2 = AlphaSet {set: 0};

		for c in comp_1.chars() {
			items_1.set(c);
		}
		for c in comp_2.chars() {
			items_2.set(c);
		}

		// Get common
		let common = items_1 & items_2;

		// Add priorities to total
		for i in 0..52 {
			if common.is_set(i) {
				total_priorities += i + 1;
			}
		}

		// Per-loop stuff
		_lines += 1;
		line.clear();
	}

	println!("Total of priorities = {total_priorities}");
}
