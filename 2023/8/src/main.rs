use std::{
	collections::{hash_map::Entry, HashMap},
	env,
	hash::BuildHasher,
	fs::File,
	io::{BufReader, BufRead},
};

/// Brent's cycle detection
///
/// Returns (cycle_length, cycle_start), AKA (lambda, mu).
fn find_cycle(start: usize, path: &Vec<char>, map: &HashMap<usize, (usize,usize)>) -> (usize, usize) {
	let mut p_it = path.iter().cycle();
	let mut p = 1;
	let mut lam = 1;
	let mut t = start;
	let mut h = match p_it.next().unwrap() {
		'L' => map.get(&start).unwrap().0,
		'R' => map.get(&start).unwrap().1,
		_ => unreachable!(),
	};

	// Find lambda
	while t != h {
		if p == lam {
			t = h;
			p *= 2;
			lam = 0;
		}

		h = match p_it.next().unwrap() {
			'L' => map.get(&h).unwrap().0,
			'R' => map.get(&h).unwrap().1,
			_ => unreachable!(),
		};
		lam += 1;
	}

	// Offset tortoise and hare
	let mut h_it = path.iter().cycle();
	let mut t_it = path.iter().cycle();
	t = start;
	h = start;
	for _ in 0..lam {
		h = match h_it.next().unwrap() {
			'L' => map.get(&h).unwrap().0,
			'R' => map.get(&h).unwrap().1,
			_ => unreachable!(),
		};
	}

	let mut mu = 0;
	while t != h {
		h = match h_it.next().unwrap() {
			'L' => map.get(&h).unwrap().0,
			'R' => map.get(&h).unwrap().1,
			_ => unreachable!(),
		};
		t = match t_it.next().unwrap() {
			'L' => map.get(&t).unwrap().0,
			'R' => map.get(&t).unwrap().1,
			_ => unreachable!(),
		};
		mu += 1;
	}

	(lam, mu)
}

fn gcd(a: usize, b: usize) -> usize {
	let mut c;
	let mut d;
	if a > b {
		d = a;
		c = b;
	} else {
		d = b;
		c = a;
	}

	let mut r = 1;
	let mut q;
	while r != 0 {
		q = d / c;
		r = d - c*q;

		d = c;
		c = r;
	}

	d
}
fn lcm(a: usize, b: usize) -> usize {
	let g = gcd(a,b);

	(a*b) / g
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

	// Get steps from top of file
	_ = reader.read_line(&mut line);
	let steps: Vec<char> = line.trim_end().chars().collect();
	//println!("Steps: {steps:?}");
	println!("Steps len {}", steps.len());

	// Get all nodes and connections
	line.clear();
	let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
	let hasher = map.hasher().clone();
	let mut start_nodes = Vec::new();
	let mut end_nodes = Vec::new();
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
		let node_hash = hasher.hash_one(node) as usize;
		// Track start and end nodes
		match node.chars().last() {
			Some('A') => start_nodes.push(node_hash),
			Some('Z') => end_nodes.push(node_hash),
			_ => {},
		}
		let (l_conn, r_conn) = conns.trim_end().split_once(',').unwrap();
		let l_conn = hasher.hash_one(l_conn.strip_prefix('(').unwrap());
		let r_conn = hasher.hash_one(r_conn.trim_start().strip_suffix(')').unwrap());

		// Map node to its connections
		match map.entry(node_hash) {
			Entry::Vacant(v) => v.insert((l_conn as usize, r_conn as usize)),
			_ => panic!("Hash collision!!!"),
		};

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	// Pre-start stuff
	let start = hasher.hash_one("AAA") as usize;
	let end = hasher.hash_one("ZZZ") as usize;
	end_nodes.sort();
	println!("{start} -> {end}");
	println!("Starts: {start_nodes:?}");
	println!("Ends: {end_nodes:?}");
	//println!("{map:?}");

	/* PART 1 & 2 */
	// Cycle offsets and lengths
	let cycles: Vec<_> = start_nodes.iter().map(|&n| find_cycle(n, &steps, &map)).collect();
	for node in start_nodes.iter() {
		let cycle = find_cycle(*node, &steps, &map);
		println!("{node} cycle props {cycle:?}");
	}
	let lcm_all = cycles.iter().fold(1, |a, &v| lcm(a,v.0));
	let lcms: Vec<_> = cycles.iter().map(|c| lcm(steps.len(), c.0)).collect();
	println!("LCM of cycles is {lcm_all}");
	println!("LCMs {lcms:?}");
	/* cycle notes
	Highest offset is a base of sorts.
	Difference of all offsets and base is how far into each cycle a given loop is.
	*/

	// Traverse map.
	let mut num_steps = 0;
	let mut cur_pos = start_nodes.clone();
	let mut cycles = Vec::new();
	loop {
		for step in steps.iter() {
			// Get next position
			for pos in cur_pos.iter_mut() {
				*pos = match step {
					'L' => map.get(pos).unwrap().0,
					'R' => map.get(pos).unwrap().1,
					_ => unreachable!(),
				};
			}

			num_steps += 1;
		}

		let ns: Vec<_> = cur_pos.iter().filter_map(|p| end_nodes.binary_search(p).ok()).collect();
		for n in ns.iter() {
			end_nodes.remove(*n);
			cycles.push(num_steps);
		}

		//if cur_pos.iter().all(|p| end_nodes.contains(p)) {
		if end_nodes.is_empty() {
			break;
		}
	}

	println!("Cycles: {cycles:?}");
	let tot = cycles.iter().copied().reduce(|a, v| lcm(a,v)).unwrap();
	println!("Total: {tot}");
	println!("Steps needed: {num_steps}");
}
