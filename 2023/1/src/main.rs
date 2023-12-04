/* Base file which contains template code for reading default input file, or
taking a file as a command-line argument, as well as looping through all lines.
*/

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Iterate a string slice returning all numeric digits and numeric representation of digit words.
///
struct DigitIter<'a>(usize, &'a str);
impl Iterator for DigitIter<'_> {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let mut it = self.1.bytes().skip(self.0);
		while self.0 < self.1.len() {
			let b = it.next().unwrap() as usize;
			let hay = &self.1[self.0 ..];
			self.0 += 1; // Increment early for simplicity

			// If digit, just return value
			if (0x30..=0x39).contains(&b){
				return Some((b - 0x30) as usize)
			}

			// Check digit names
			if hay.strip_prefix("one").is_some() {
				return Some(1)
			} else if hay.strip_prefix("two").is_some() {
				return Some(2)
			} else if hay.strip_prefix("six").is_some() {
				return Some(6)
			} else if hay.strip_prefix("four").is_some() {
				return Some(4)
			} else if hay.strip_prefix("five").is_some() {
				return Some(5)
			} else if hay.strip_prefix("nine").is_some() {
				return Some(9)
			} else if hay.strip_prefix("three").is_some() {
				return Some(3)
			} else if hay.strip_prefix("seven").is_some() {
				return Some(7)
			} else if hay.strip_prefix("eight").is_some() {
				return Some(8)
			}
		}

		None
	}

	fn last(self) -> Option<Self::Item> {
		let mut it = self.1.bytes().rev();
		for i in (0 .. self.1.len()).rev() {
			let b = it.next().unwrap() as usize;
			let hay = &self.1[i ..];

			// If digit, just return value
			if (0x30..=0x39).contains(&b){
				return Some((b - 0x30) as usize)
			}

			// Check digit names
			if hay.strip_prefix("one").is_some() {
				return Some(1)
			} else if hay.strip_prefix("two").is_some() {
				return Some(2)
			} else if hay.strip_prefix("six").is_some() {
				return Some(6)
			} else if hay.strip_prefix("four").is_some() {
				return Some(4)
			} else if hay.strip_prefix("five").is_some() {
				return Some(5)
			} else if hay.strip_prefix("nine").is_some() {
				return Some(9)
			} else if hay.strip_prefix("three").is_some() {
				return Some(3)
			} else if hay.strip_prefix("seven").is_some() {
				return Some(7)
			} else if hay.strip_prefix("eight").is_some() {
				return Some(8)
			}
		}

		None
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
	let mut sum: usize = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* PART 1 */
		// Iterate chars/bytes and extract all digits.
		/*
		let mut it = line.bytes().filter_map(|b| match b {
			0x30..=0x39 => Some((b - 0x30) as usize),
			_ => None,
		});
		let front = it.next().unwrap();
		let back = match it.last() {
			Some(v) => v,
			None => front,
		};

		sum += front * 10 + back;
		*/

		/* PART 2 */
		let mut di = DigitIter(0, &line[..]);
		let front = di.next().unwrap();
		let back = match di.last() {
			Some(v) => v,
			None => front,
		};

		sum += front * 10 + back;

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Sum of all lines is: {sum}");
}
