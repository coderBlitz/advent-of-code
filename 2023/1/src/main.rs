/* Base file which contains template code for reading default input file, or
taking a file as a command-line argument, as well as looping through all lines.
*/

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

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
	let mut sum = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		// Iterate line
		let mut it = line.chars().filter(|c| c.is_digit(10));
		let front = it.next().unwrap().to_digit(10).unwrap();
		let back = match it.last() {
			Some(d) => d.to_digit(10).unwrap(),
			None => front,
		};

		sum += front * 10 + back;

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Sum of all lines is: {sum}");
}
