use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

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
	// Go till empty line to count total stacks (and items)
	let mut crate_count = 0;
	let mut stack_count = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		} else if line.trim().len() == 0 {
			break;
		}

		// Get line elements count
		let count = line.split_whitespace().count();

		// Add to crate count regardless
		crate_count += count;

		// Increase stack count to maximum elements encountered in a single line
		if count > stack_count {
			stack_count = count;
		}

		// Per-iteration things
		line.clear();
	}

	let crate_count = crate_count - stack_count; // Adjust for line with stack numbers
	println!("Found {crate_count} crates across {stack_count} stacks");

	// # of chars used up by each stack column (TODO: Calculate assuming identical count per stack)
	let chars_per_stack = 4; // Challenge has 1-letter crate names, +2 brackets, and a space

	/* Reset reader, then do remaining collection and processing
	*/
	reader.seek(SeekFrom::Start(0)).expect("Seek shouldn't fail");

	// First collect items into their stacks
	let mut stacks: Vec<Vec<String>> = Vec::with_capacity(stack_count);
	stacks.resize(stack_count, Vec::new());

	let mut _lines = 0;
	line.clear();
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		} else if line.trim().len() == 0 {
			break;
		}

		// Iterate over crates in line
		let starts = line.match_indices('[').map(|(idx,_)| idx);
		let ends = line.match_indices(']').map(|(idx,_)| idx);
		for (start, end) in std::iter::zip(starts, ends) {
			let label = &line[start+1..end];
			let stack = start / chars_per_stack;
			//println!("Line {_lines}: Found crate {label} on stack {stack}");

			stacks[stack].push(label.to_string());
		}

		// Iterate stuff
		_lines += 1;
		line.clear();
	}

	// Reverse order
	for i in 0..stacks.len() {
		let stack = &stacks[i];
		println!("Stack {i} init size = {}", stack.len());
		stacks[i].reverse();
	}

	/* Do movement processing
	*/
	line.clear();
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		// Grab all 3 values
		let nums: Vec<&str> = line.split_whitespace().collect();
		let n_to_move: usize = nums[1].parse().unwrap();
		let src_stack: usize = nums[3].parse::<usize>().unwrap() - 1; // 0-indexed
		let dst_stack: usize = nums[5].parse::<usize>().unwrap() - 1;

		println!("Moving {n_to_move} crates from {src_stack} to {dst_stack}");
		// Do movement
		for _ in 0..n_to_move {
			let grab = stacks[src_stack].pop().unwrap();
			stacks[dst_stack].push(grab);
		}

		// Iterate stuff
		_lines += 1;
		line.clear();
	}

	let mut res = String::new();
	for (i, stack) in stacks.iter().enumerate() {
		println!("Stack {i} final size = {}. Top is {}", stack.len(), stack.last().unwrap());
		res.push(stack.last().unwrap().as_bytes()[0] as char);
	}

	println!("res = {res}");
}
