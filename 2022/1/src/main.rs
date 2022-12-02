#[derive(Default, Copy, Clone)]
struct FullElf {
	_idx: usize,
	calories: usize,
}

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
	const TOP_N: usize = 3;
	let mut inventories = Vec::<FullElf>::new();

	let argv: Vec<String> = env::args().collect();

	let filename;
	if argv.len() > 1 {
		filename = argv[1].clone();
	} else {
		filename = String::from("./input");
	}

	
	let file = File::open(filename).expect("Could not open file");
	let mut reader = BufReader::new(file);

	let mut line = String::new();
	let mut sum: usize = 0;
	let mut idx: usize = 0;
	let mut lines = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		if let Ok(val) = line.trim().parse::<usize>() {
			sum += val;
		} else {
			// Check for new max, then add to next position in inventories vector
			inventories.push(FullElf {_idx: idx, calories: sum });

			// Set for next
			idx += 1;
			sum = 0;
		}

		lines += 1;
		line.clear();
	}
	println!("{lines} lines read");
	println!("{} inventories summed", idx + 1);

	// Sort only if there are more inventories than desired for  sum
	let start: usize;
	if inventories.len() >= TOP_N {
		start = inventories.len() - TOP_N;
		inventories.sort_unstable_by_key(|k| k.calories);
	} else {
		start = 0;
	}

	sum = 0;
	for elf in &inventories[start..] {
		println!("elf {} has {} calories", elf._idx, elf.calories);
		sum += elf.calories;
	}

	println!("Calories of top {TOP_N} elves is {}", sum);
}
