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
	const _RED_MAX: usize = 12;
	const _GREEN_MAX: usize = 13;
	const _BLUE_MAX: usize = 14;

	let mut line = String::new();
	let mut _lines = 0;
	let mut sum = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* PART 1 */
		/*let colon_idx = line.find(':').unwrap();
		let (game, reveals) = line.split_at(colon_idx);
		let game_num: usize = game.strip_prefix("Game ").unwrap().parse().unwrap();
		let mut valid = true;
		'reveal_loop: for reveal in reveals.split_terminator(';') {
			for entry in reveal.split_terminator(',') {
				let mut t = entry.trim_start_matches(&[':',' ']).split_whitespace();
				let count: usize = t.next().unwrap().parse().unwrap();
				let color = t.next().unwrap();
				
				valid = match color {
					"red" => count <= RED_MAX,
					"green" => count <= GREEN_MAX,
					"blue" => count <= BLUE_MAX,
					_ => panic!(),
				};
				if !valid {
					break 'reveal_loop
				}
			}
		}

		// Add if valid
		if valid {
			sum += game_num;
		}*/

		/* PART 2 */
		let colon_idx = line.find(':').unwrap();
		let (_, reveals) = line.split_at(colon_idx);
		let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
		for reveal in reveals.split_terminator(';') {
			for entry in reveal.split_terminator(',') {
				let mut t = entry.trim_start_matches(&[':',' ']).split_whitespace();
				let count: usize = t.next().unwrap().parse().unwrap();
				let color = t.next().unwrap();

				match color {
					"red" => min_red = min_red.max(count),
					"green" => min_green = min_green.max(count),
					"blue" => min_blue = min_blue.max(count),
					_ => panic!(),
				};
			}
		}
		let power = min_red * min_green * min_blue;
		sum += power;

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Valid game sum: {sum}");
}
