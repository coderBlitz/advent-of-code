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
	let tree_vec = &mut Vec::new();
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		let trimmed = line.trim();
		for c in trimmed.chars() {
			match c {
				'0'..='9' => tree_vec.push(c as u32 - '0' as u32),
				_ => println!("c is {}", c as u32),
			};
		}

		// Per-iteration things
		_lines += 1;
		line.clear();
	}
	println!("rows/lines = {_lines}");
	println!("Trees = {}", tree_vec.len());

	let row_size = tree_vec.len() / _lines;

	let visible_vec = &mut Vec::new();
	visible_vec.reserve_exact(tree_vec.len());
	visible_vec.resize(tree_vec.len(), 0);

	// Iterate
	for i in 1..(_lines-1) {
		let row = row_size * i;
		let col = i;

		// Iterate columns left-to-right tracking largest visible
		let mut row_max = tree_vec[row];
		for j in 1..(row_size-1) {
			let idx = row + j;

			if tree_vec[idx] > row_max {
				visible_vec[idx] += 1;
				row_max = tree_vec[idx];
			}
		}

		// Iterate right-to-left
		row_max = tree_vec[row+row_size-1];
		for j in (1..(row_size-1)).rev() {
			let idx = row + j;

			if tree_vec[idx] > row_max {
				visible_vec[idx] += 1;
				row_max = tree_vec[idx];
			}
		}

		// Iterate top-to-bottom
		let mut col_max = tree_vec[col];
		for j in 1..(row_size-1) {
			let idx = col + j * row_size;

			if tree_vec[idx] > col_max {
				visible_vec[idx] += 1;
				col_max = tree_vec[idx];
			}
		}

		// Iterate bottom-to-top
		col_max = tree_vec[row_size * (row_size - 1) + col];
		for j in (1..(row_size-1)).rev() {
			let idx = col + j * row_size;

			if tree_vec[idx] > col_max {
				visible_vec[idx] += 1;
				col_max = tree_vec[idx];
			}
		}
	}

	// Count
	let mut trees_visible = 4 * (row_size - 1); // All outer are visible
	for &i in visible_vec.iter() {
		if i > 0 {
			trees_visible += 1;
		}
	}

	println!("Total visible = {trees_visible}");
}
