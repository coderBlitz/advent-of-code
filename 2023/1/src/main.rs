/* Base file which contains template code for reading default input file, or
taking a file as a command-line argument, as well as looping through all lines.
*/

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct DigitIter<'a>(usize, &'a str);
impl Iterator for DigitIter<'_> {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		let mut it = self.1.bytes().skip(self.0);
		let len = self.1.len();
		for i in self.0 .. self.1.len() {
			self.0 += 1; // Increment early for simplicity

			let b = it.next().unwrap() as usize;
			let short_slice = &self.1[i .. (i + 3).min(len)];
			let mid_slice = &self.1[i .. (i + 4).min(len)];
			let long_slice = &self.1[i .. (i + 5).min(len)];

			// If digit, just return value
			if (0x30..=0x39).contains(&b){
				return Some((b - 0x30) as usize)
			}

			// Check short digit names
			if short_slice.find("one").is_some() {
				return Some(1)
			} else if short_slice.find("two").is_some() {
				return Some(2)
			} else if short_slice.find("six").is_some() {
				return Some(6)
			} else if mid_slice.find("four").is_some() {
				return Some(4)
			} else if mid_slice.find("five").is_some() {
				return Some(5)
			} else if mid_slice.find("nine").is_some() {
				return Some(9)
			} else if long_slice.find("three").is_some() {
				return Some(3)
			} else if long_slice.find("seven").is_some() {
				return Some(7)
			} else if long_slice.find("eight").is_some() {
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
