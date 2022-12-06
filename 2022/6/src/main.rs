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

	#[allow(dead_code)]
	fn clear(&mut self) {
		self.set = 0;
	}

	fn is_set(&self, idx: u64) -> bool {
		if idx > 63 {
			panic!("Cannot shift more than 63!");
		}

		(self.set & (0x1 << idx)) > 0
	}

	fn count_set(&self) -> u32 {
		self.set.count_ones()
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
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* Part 1
		*/
		const UNIQ: usize = 4;
		let mut set = Vec::with_capacity(UNIQ);
		let mut line_enum = line.chars().enumerate();
		while let Some((_,c)) = line_enum.next() {
			set.push(c);

			if set.len() == UNIQ {
				break;
			}
		}

		let mut sop = UNIQ; // start-of-packet cannot be less than UNIQ
		let mut check = AlphaSet {set: 0};
		for (pos,c) in line_enum {
			// Add vector values to bit set
			for t in &set {
				check.set(*t);
			}

			// Check if unique
			if check.count_set() as usize == UNIQ {
				// Success!
				sop = pos;
				break;
			}

			// Insert new character
			set[pos % UNIQ] = c;

			// Reset for next loop
			check.clear();
		}
		println!("SOP = {sop}");

		// Per-iteration things
		_lines += 1;
		line.clear();
	}
}
