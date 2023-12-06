/* Base file which contains template code for reading default input file, or
taking a file as a command-line argument, as well as looping through all lines.
*/

use std::{
	collections::{btree_map::Entry, BTreeMap},
	env,
	fs::File,
	io::{BufReader, BufRead},
	ops::Range,
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
	let mut buf = String::new();
	let mut _lines = 0;

	// Get seeds from top of file
	_ = reader.read_line(&mut buf);
	let mut nums: Vec<usize> = buf.strip_prefix("seeds: ").unwrap()
		.trim()
		.split_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();
	nums.sort();

	// Iterate rest of file with maps
	buf.clear();
	let mut new_map = BTreeMap::new();
	new_map.extend(nums.iter().map(|v| (v,v)));
	while let Ok(length) = reader.read_line(&mut buf) {
		let line = buf.trim();
		if length == 0 {
			break;
		} else if line.len() == 0 {
			_lines += 1;
			buf.clear();
			continue;
		} else if line.contains(':') {
			// Insert unmapped nums
			for &n in nums.iter() {
				if let Entry::Vacant(v) = new_map.entry(n) {
					v.insert(n);
				};
			}

			// Clear nums and insert mapped values
			nums.clear();
			nums.extend(new_map.iter().map(|(_,&v)| v));
			new_map.clear();
			println!("Nums now: {nums:?}");

			_lines += 1;
			buf.clear();
			continue;
		}


		// Get range numbers
		let mut it = line.split_whitespace();
		let dst_s: usize = it.next().unwrap().parse().unwrap();
		let src_s: usize = it.next().unwrap().parse().unwrap();
		let len: usize = it.next().unwrap().parse().unwrap();

		let src_range = Range { start: src_s, end: src_s + len };
		println!("Src range: {src_range:?}");

		// Map
		for n in nums.iter_mut() {
			if src_range.contains(n) {
				if let Entry::Vacant(v) = new_map.entry(*n) {
					v.insert(dst_s + (*n - src_s));
				};
			}
		}

		// Per-iteration things
		_lines += 1;
		buf.clear();
	}

	// Insert unmapped nums
	for &n in nums.iter() {
		if let Entry::Vacant(v) = new_map.entry(n) {
			v.insert(n);
		};
	}

	// Clear nums and insert mapped values
	nums.clear();
	nums.extend(new_map.iter().map(|(_,&v)| v));
	new_map.clear();

	println!("Final nums: {nums:?}");
	println!("Min is {}", nums.iter().min().unwrap());
}
