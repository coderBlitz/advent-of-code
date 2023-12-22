use std::{
	collections::BTreeSet,
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
	let mut lines = 0;
	let mut galaxies: Vec<(isize, isize)> = Vec::new();
	let mut empty_rows: Vec<isize> = Vec::new();
	let mut nonempty_cols: BTreeSet<isize> = BTreeSet::new();
	let mut width = 0;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		width = line.trim_end().len();
		let gs = line.trim_end().bytes()
			.enumerate()
			.filter_map(|(i, v)| match v {
				b'#' => Some((lines, i as isize)),
				_ => None,
			})
			.inspect(|(_,c)| { nonempty_cols.insert(*c); });
		let before_len = galaxies.len();
		galaxies.extend(gs);
		if galaxies.len() == before_len {
			empty_rows.push(lines);
		}

		// Per-iteration things
		lines += 1;
		line.clear();
	}

	let empty_cols: Vec<isize> = (0isize .. width as isize).filter(|n| !nonempty_cols.contains(n)).collect();

	println!("Galaxies: {galaxies:?}");
	println!("Empty rows: {empty_rows:?}");
	println!("Empty cols: {empty_cols:?}");

	/* PART 1 */
	// Shortest path is just Manhattan distance between two points, plus expansion term.
	let mut sum = 0;
	for i in 0 .. galaxies.len() {
		for j in (i+1) .. galaxies.len() {
			// Manhattan calculation (plus final step to reach galaxy)
			let dist = (galaxies[j].0 - galaxies[i].0) + (galaxies[j].1 - galaxies[i].1).abs();

			// Find row expansion term
			let row_lower = match empty_rows.binary_search(&galaxies[i].0) {
				Ok(v) => v,
				Err(v) => v,
			};
			let row_upper = match empty_rows.binary_search(&galaxies[j].0) {
				Ok(v) => v,
				Err(v) => v,
			};
			let row_expansion = row_upper - row_lower;

			// Find col expansion
			fn min_max(a: isize, b: isize) -> (isize, isize) {
				if a < b {
					(a,b)
				} else {
					(b,a)
				}
			}
			let (low, high) = min_max(galaxies[i].1, galaxies[j].1);
			let col_lower = match empty_cols.binary_search(&low) {
				Ok(v) => v,
				Err(v) => v,
			};
			let col_upper = match empty_cols.binary_search(&high) {
				Ok(v) => v,
				Err(v) => v,
			};
			let col_expansion = col_upper - col_lower;

			//println!("Row expansion: {row_expansion}");
			//println!("Col expansion: {col_expansion}");
			//println!("<{:?}, {:?}> = {dist}", galaxies[i], galaxies[j]);

			sum += dist + (row_expansion as isize)*999_999 + (col_expansion as isize)*999_999;
		}
	}

	println!("Dist sum: {sum}");
}
