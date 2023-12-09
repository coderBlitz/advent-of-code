use std::{
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

		// Get numbers
		let mut nums: Vec<isize> = line.trim_end().split_whitespace().map(|v| v.parse().unwrap()).collect();
		//println!("{nums:?}");

		// Compute down the line
		let mut max = nums.len()-1;
		while max > 1 && !nums.iter().enumerate().filter(|(i,_)| i < &max).all(|(_, &v)| v == 0) {
			for i in 0..max {
				nums[i] = nums[i+1] - nums[i];
			}
			//println!("{max} {nums:?}");
			max -= 1;
		}
		let tot: isize = nums.iter().copied().sum();
		//println!("Total {tot}");
		sum += tot;

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Sum {sum}");
}
