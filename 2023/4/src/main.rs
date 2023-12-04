/* Base file which contains template code for reading default input file, or
taking a file as a command-line argument, as well as looping through all lines.
*/

use std::{
	collections::BTreeSet,
	env,
	fs::File,
	io::{BufReader, BufRead},
};

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

		/* PART 1 */
		let (_, numbers) = line.split_once(':').unwrap();
		let (winners, numbers) = numbers.split_once('|').unwrap();
		let winners: BTreeSet<usize> = winners.trim().split_whitespace()
			.map(|n| n.parse().unwrap())
			.collect();
		let numbers: BTreeSet<usize> = numbers.trim().split_whitespace()
			.map(|n| n.parse().unwrap())
			.collect();

		let common = numbers.intersection(&winners).count();
		if common > 0 {
			sum += 2_usize.pow((common - 1) as u32);
		}

		// Per-iteration things
		_lines += 1;
		line.clear();
	}
	println!("Sum = {sum}");
}
