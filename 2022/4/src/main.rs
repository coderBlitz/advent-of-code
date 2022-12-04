use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

// Won't use std::ops::Range because inclusive bounds is easier for this problem
// start <= end
struct Range {
	start: usize,
	end: usize,
}

impl Range {
	// If two ranges overlap at all
	fn overlaps(&self, other: &Self) -> bool {
		other.contains(self)
		|| (self.start <= other.start && other.start <= self.end)
		|| (self.start <= other.end && other.end <= self.end)
	}

	// If this range fully contains another range (superset, non-proper)
	fn contains(&self, other: &Self) -> bool {
		self.start <= other.start && other.end <= self.end
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
	let mut total_supersets = 0;
	let mut total_overlaps = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* Part 1
		*/
		let elf_ranges: Vec<&str> = line.split(',').collect();
		let elf_1_range: Vec<usize> = elf_ranges[0].trim().split('-').map(|x| x.parse().expect("Invalid value")).collect();
		let elf_2_range: Vec<usize> = elf_ranges[1].trim().split('-').map(|x| x.parse().expect("Invalid value")).collect();

		let elf_1_range = Range { start: elf_1_range[0], end: elf_1_range[1] };
		let elf_2_range = Range { start: elf_2_range[0], end: elf_2_range[1] };

		if elf_1_range.contains(&elf_2_range) || elf_2_range.contains(&elf_1_range) {
			total_supersets += 1;
		}

		/* Part 2
		*/
		if elf_1_range.overlaps(&elf_2_range) {
			total_overlaps += 1;
		}

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Total supersets = {total_supersets}");
	println!("Total overlaps = {total_overlaps}");
}
