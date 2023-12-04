use std::{
	env,
	fs::File,
	io::{BufReader, BufRead},
	ops::Range,
};

#[derive(Clone, Debug)]
struct Entry {
	typ: EntryType,
	span: Range<usize>,
}
#[derive(Clone, Debug)]
enum EntryType {
	Sym(char, Vec<usize>), // Adjacent entries
	Num(usize),
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
	let mut sum = 0;
	let mut prev_row: Vec<Entry> = Vec::new();
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* PART 1 */
		// Parse entries in current row
		let mut cur_row: Vec<Entry> = Vec::new();
		let mut idx: usize = 0;
		let mut hay = line[..].trim_end();
		loop {
			if hay.is_empty() {
				break;
			}

			// Set entry string based on first char.
			let ent_str = match hay.chars().nth(0).unwrap() {
				'.' => {
					hay = &hay[1..];
					idx += 1;
					continue;
				},
				'0'..='9' => hay.split_terminator(|c: char| !c.is_digit(10)).next().unwrap(),
				_ => &hay[0..1],
			};
				
			// Try to parse entry as number and push number if successful, else push symbol.
			cur_row.push(match ent_str.parse::<usize>() {
				Ok(v) => Entry {
					typ: EntryType::Num(v),
					span: Range {start: idx.saturating_sub(1), end: (idx + ent_str.len()).min(line.len()-2)},
				},
				Err(_) => Entry {
					typ: EntryType::Sym(ent_str.chars().nth(0).unwrap(), Vec::new()),
					span: Range {start: idx, end: idx},
				},
			});

			// Shift haystack past parsed entry
			idx += ent_str.len();
			hay = &hay[ent_str.len()..];
		}
		//println!("Line: {}", line.trim_end());
		//println!("{cur_row:?}");

		/* Solve puzzle
		*/
		fn overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
			a.start <= b.end && b.start <= a.end
		}

		// Iterate current & previous rows
		for c in cur_row.iter_mut() {
			for p in prev_row.iter_mut() {
				match p.typ {
					EntryType::Sym(_, ref mut adj) => { if let EntryType::Num(v) = c.typ {
						if overlap(&p.span, &c.span) {
							adj.push(v);
						}
					}},
					EntryType::Num(v) => { if let EntryType::Sym(_, ref mut adj) = c.typ {
						if overlap(&p.span, &c.span) {
							adj.push(v);
						}
					}},
				};
			}
		}

		let mut i = 0;
		while i < cur_row.len() {
			let c_span = cur_row[i].span.clone();
			// If space on left
			if i > 0 {
				let c1_span = cur_row[i-1].span.clone();
				if let EntryType::Num(v) = cur_row[i-1].typ {
					if let EntryType::Sym(_, ref mut adj) = cur_row[i].typ {
						if overlap(&c1_span, &c_span) {
							adj.push(v);
						}
					}
				}
			}

			// If space on right
			if i < cur_row.len().saturating_sub(1) {
				let c1_span = cur_row[i+1].span.clone();
				if let EntryType::Num(v) = cur_row[i+1].typ {
					if let EntryType::Sym(_, ref mut adj) = cur_row[i].typ {
						if overlap(&c1_span, &c_span) {
							adj.push(v);
						}
					}
				}
			}

			i += 1;
		}

		/* PART 2 */
		// Check previous row symbols
		let gear_total: usize = prev_row.iter().filter_map(|e| {
			match e.typ {
				EntryType::Sym(c, ref v) => {
					if c == '*' && v.len() == 2 {
						Some(v.iter().product::<usize>())
					} else { None }
				},
				_ => None,
			}
		}).sum();
		sum += gear_total;

		// Migrate leftover to next previous
		prev_row = cur_row;

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Sum = {sum}");
}
