use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct RegState {
	cycle: usize,
	value: isize,
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
	let x_cycles = &mut Vec::new();
	let current_state = &mut RegState { cycle: 1, value: 1 }; // Default register state

	x_cycles.push(*current_state);
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* Part 1
		Instead of simulating individual clock cycles, we will simulate instructions
		 and update the cycle count based on instructions run.
		So when addx is run, we add 2 cycles to the current cycle counter/variable.
		When noop is run, we add 1 cycle.
		*/
		let command: Vec<&str> = line.trim().split_whitespace().collect();
		match command[0] {
			"noop" => current_state.cycle += 1,
			"addx" => {
				current_state.cycle += 2;
				current_state.value += command[1].parse::<isize>().unwrap();
			},
			_ => eprintln!("Unrecognized instruction {}", command[0]),
		};

		x_cycles.push(*current_state);

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("state changes = {}", x_cycles.len());
	println!("Final cycle = {}; x value = {}", current_state.cycle, current_state.value);

	let finds: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
	let mut val_sum = 0;
	for f in finds.iter() {
		let idx = match x_cycles.binary_search_by_key(f, |x| x.cycle) {
			Ok(i) => i,
			Err(i) => i - 1,
		};
		let state = x_cycles[idx];
		val_sum += state.value * (*f as isize);
		println!("cycle = {}, value = {}", state.cycle, state.value);
	}

	println!("sum = {val_sum}");

	/* Part 2
	*/
	for row in 0..6 {
		let pixels = &mut String::with_capacity(40);
		for col in 1..=40 {
			let pos = row * 40 + col;

			// Grab state for current cycle position
			let idx = match x_cycles.binary_search_by_key(&pos, |x| x.cycle) {
				Ok(i) => i,
				Err(i) => i.checked_sub(1).unwrap_or(i),
			};
			let state = x_cycles[idx];
			//println!("{pos} --- {}", state.value);

			// Check if pixel drawn
			if (state.value - (col-1) as isize).abs() <= 1 {
				pixels.push('#');
			} else {
				pixels.push('.');
			}
		}

		println!("{pixels}");
		pixels.clear();
	}
}
