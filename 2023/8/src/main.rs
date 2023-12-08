use std::{
	collections::{hash_map::Entry, HashMap},
	env,
	hash::BuildHasher,
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

	// Get steps from top of file
	_ = reader.read_line(&mut line);
	let steps: Vec<char> = line.trim_end().chars().collect();
	println!("Steps: {steps:?}");

	// Get all nodes and connections
	line.clear();
	let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
	let mut hasher = map.hasher().clone();
	while let Ok(length) = reader.read_line(&mut line) {
		let buf = line.trim_end();
		if length == 0 {
			break;
		} else if buf.len() == 0 {
			_lines += 1;
			line.clear();
			continue;
		}

		// Split line
		let (node, conns) = line.split_once(" = ").unwrap();
		let node = hasher.hash_one(node);
		let (l_conn, r_conn) = conns.trim_end().split_once(',').unwrap();
		let l_conn = hasher.hash_one(l_conn.strip_prefix('(').unwrap());
		let r_conn = hasher.hash_one(r_conn.trim_start().strip_suffix(')').unwrap());

		match map.entry(node as usize) {
			Entry::Vacant(v) => v.insert((l_conn as usize, r_conn as usize)),
			_ => panic!("Hash collision!!!"),
		};

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	let start = hasher.hash_one("AAA") as usize;
	let end = hasher.hash_one("ZZZ") as usize;
	println!("{start} -> {end}");
	//println!("{map:?}");

	/* PART 1 */
	// Traverse map.
	let mut num_steps = 0;
	let mut cur_pos = start;
	for step in steps.iter().cycle() {
		//println!("{num_steps} {cur_pos}");
		if cur_pos == end {
			break;
		}

		// Get next position
		cur_pos = match step {
			'L' => map.get(&cur_pos).unwrap().0,
			'R' => map.get(&cur_pos).unwrap().1,
			_ => unreachable!(),
		};

		num_steps += 1;
	}

	println!("Steps needed: {num_steps}");
}
