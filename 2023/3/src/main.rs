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
#[derive(Clone, Copy, Debug)]
enum EntryType {
	Sym,
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
					typ: EntryType::Sym,
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

		// Iterate current row
		let mut c = 0;
		let mut p = 0;
		let mut last_ent: Option<Entry> = None;
		while c < cur_row.len() {
			// While c is ahead, move p up.
			while p < prev_row.len() && prev_row[p].span.end < cur_row[c].span.start {
				p += 1;
			}

			//println!("Previou {p} : {prev_row:?}");
			//println!("Current {c} : {cur_row:?}");

			// Only handle symbol entries in previous row. Sum then remove values if overlapped.
			if let EntryType::Num(v) = cur_row[c].typ {
				// Check against current row
				if let Some(ref e) = last_ent { if let EntryType::Sym = e.typ {
					if overlap(&e.span, &cur_row[c].span) {
						sum += v;
						cur_row.remove(c);
						last_ent = None; // Clear last entry since current gets removed
						continue;
					}
				}}

				// Check against previous row
				if p < prev_row.len() {
					if let EntryType::Sym = prev_row[p].typ {
						if overlap(&prev_row[p].span, &cur_row[c].span) {
							//println!("{:?} and {:?} overlap!", prev_row[p], cur_row[c]);
							sum += v;
							cur_row.remove(c);
							continue;
						}
					};
				}
			}

			// Remember last symbol span in current row
			if let EntryType::Sym = cur_row[c].typ {
				// If symbol found after number
				if let Some(ref e) = last_ent { if let EntryType::Num(v) = e.typ {
					if overlap(&e.span, &cur_row[c].span) {
						sum += v;
						last_ent = Some(cur_row[c].clone()); // Done here since continue skips below logic
						cur_row.remove(c-1); // Previous entry (since symbol is after number)
						continue;
					}
				}}
			}

			last_ent = Some(cur_row[c].clone());

			// Increment if p overlaps, since top loop stops upon overlap (and can't iterate beyond it).
			if p < prev_row.len() && overlap(&prev_row[p].span, &cur_row[c].span) {
				p += 1;
			} else {
				c += 1;
			}
		}

		// Iterate previous row
		c = 0;
		p = 0;
		while p < prev_row.len() {
			// While p is ahead, move c up.
			while c < cur_row.len() && cur_row[c].span.end < prev_row[p].span.start {
				c += 1;
			}

			// Only handle symbol entries in current row. Sum then remove values if overlapped.
			if c < cur_row.len() {
				if let EntryType::Sym = cur_row[c].typ {
					if let EntryType::Num(v) = prev_row[p].typ {
						if overlap(&prev_row[p].span, &cur_row[c].span) {
							//println!("{:?} and {:?} overlap!", prev_row[p], cur_row[c]);
							sum += v;
							prev_row.remove(p);
							continue;
						}
					};
				}
			}

			// Increment if c overlaps, since top loop stops upon overlap (and can't iterate beyond it).
			if c < cur_row.len() && overlap(&prev_row[p].span, &cur_row[c].span) {
				c += 1;
			} else {
				p += 1;
			}
		}

		// Migrate leftover to next previous
		prev_row = cur_row;

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Sum = {sum}");
}
