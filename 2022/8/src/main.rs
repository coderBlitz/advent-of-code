use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn scenic_score(trees: &Vec<u32>, row_size: usize, idx: usize) -> usize {
	let (base_row, base_col) = (idx / row_size, idx % row_size);

	let base_height = trees[idx];

	// Check left
	let mut left_views = 0;
	for i in (0..base_col).rev() {
		let pos = base_row * row_size + i;

		left_views += 1;
		if trees[pos] >= base_height {
			break;
		}
	}

	// Check right
	let mut right_views = 0;
	for i in (base_col+1)..row_size {
		let pos = base_row * row_size + i;

		right_views += 1;
		if trees[pos] >= base_height {
			break;
		}
	}

	// Check up
	let mut up_views = 0;
	for i in (0..base_row).rev() {
		let pos = i * row_size + base_col;

		up_views += 1;
		if trees[pos] >= base_height {
			break;
		}
	}

	// Check down
	let mut down_views = 0;
	for i in (base_row+1)..row_size {
		let pos = i * row_size + base_col;

		down_views += 1;
		if trees[pos] >= base_height {
			break;
		}
	}

	//println!("{left_views} {right_views} {up_views} {down_views}");
	left_views * right_views * up_views * down_views
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

	/* Part 1
	*/
	let mut trees_visible = 4 * (row_size - 1); // All outer are visible
	for &i in visible_vec.iter() {
		if i > 0 {
			trees_visible += 1;
		}
	}

	println!("Total visible = {trees_visible}");

	/* Part 2
	*/
	// Iterate
	let mut max_scenic = 0;
	for i in 1..(_lines-1) {
		let row = row_size * i;

		for j in 1..(row_size-1) {
			let idx = row + j;

			let scenic = scenic_score(tree_vec, row_size, idx);
			if scenic > max_scenic {
				max_scenic = scenic;
			}
		}
	}

	println!("Max scenic = {max_scenic}");
}
