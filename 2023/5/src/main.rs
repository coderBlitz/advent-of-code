/* Base file which contains template code for reading default input file, or
taking a file as a command-line argument, as well as looping through all lines.
*/

use std::{
	env,
	fs::File,
	io::{BufReader, BufRead},
	ops::Range,
};

fn overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
	a.start <= b.end && b.start <= a.end
}
fn intersect_range(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
	if !overlap(a, b) {
		return None
	}

	let start = a.start.max(b.start);
	let end = a.end.min(b.end);
	match start <= end {
		true => Some(Range { start, end }),
		false => None,
	}
}
fn union_range(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
	if !overlap(a, b) {
		return None
	}

	let start = a.start.min(b.start);
	let end = a.end.max(b.end);
	match start <= end {
		true => Some(Range { start, end }),
		false => None,
	}
}
fn symdif_range(a: &Range<usize>, b: &Range<usize>) -> Option<(Range<usize>, Option<Range<usize>>)> {
	let intr = intersect_range(a,b)?;
	let onion = union_range(a,b).unwrap(); // Intersect succeeded, so union will succeed

	// Check if 1 or 2 ranges result
	if intr == onion {
		None
	} else if onion.start == intr.start {
		// 1 range
		Some((Range { start: intr.end, end: onion.end }, None))
	} else if onion.end == intr.end {
		// 1 range
		Some((Range { start: onion.start, end: intr.start }, None))
	} else {
		// 2 ranges
		Some((Range { start: onion.start, end: intr.start }, Some(Range { start: intr.end, end: onion.end })))
	}
}

// Count elements covered by all range elements (naive, ranges must not overlap).
fn range_count(a: &[Range<usize>]) -> usize {
	a.iter().map(|r| r.end - r.start).sum()
}

fn combine_ranges(v: &mut Vec<Range<usize>>) {
	v.sort_by_key(|r| r.start);
	// Combine overlapping ranges in-place.
	if v.len() > 0 {
		let mut k = 0;
		let mut cur_rng = v[0].clone();
		for i in 1..v.len() {
			// Set scratch range to union of scratch and current, or current if not overlapping.
			cur_rng = match union_range(&cur_rng, &v[i]) {
				Some(r) => r,
				None => {
					v[k] = cur_rng;
					k += 1;
					v[i].clone()
				},
			}
		}
		v[k] = cur_rng; // Add remainder range
		k += 1;
		v.truncate(k);
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
	let mut buf = String::new();
	let mut _lines = 0;

	// Get seeds from top of file
	_ = reader.read_line(&mut buf);
	let nums: Vec<usize> = buf.strip_prefix("seeds: ").unwrap()
		.trim()
		.split_whitespace()
		.map(|n| n.parse().unwrap())
		.collect();

	let i1 = nums.iter().skip(1).step_by(2);
	let mut ranges: Vec<Range<usize>> = nums.iter().step_by(2).zip(i1)
		.map(|(&s, &l)| Range {
			start: s,
			end: s+l
		}).collect();
	ranges.sort_by_key(|r| r.start);
	println!("Ranges {ranges:?}");
	println!("Coverage {}", range_count(&ranges[..]));
	let coverage = range_count(&ranges[..]);

	buf.clear();
	println!("Loop start");
	let mut new_ranges = Vec::new();
	while let Ok(length) = reader.read_line(&mut buf) {
		let line = buf.trim();
		if length == 0 {
			break;
		} else if line.len() == 0 {
			_lines += 1;
			buf.clear();
			continue;
		} else if line.contains(':') {
			println!("----- RESET -----");

			// Combine working and scratch vector.
			// Sort combined ranges (by start value).
			// Combine overlapping ranges between.
			// Clear scratch vector.
			ranges.extend(new_ranges.drain(..)); // Combine
			combine_ranges(&mut ranges);
			assert_eq!(coverage, range_count(&ranges[..])); // If this panics, we lost/gained seeds.

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
		let _dst_range = Range { start: dst_s, end: dst_s + len };
		println!("Map {src_range:?} --> {_dst_range:?}");

		/*
		*/
		let mut i = 0;
		while i < ranges.len() {
			if let Some(r) = intersect_range(&ranges[i], &src_range) {
				new_ranges.push(Range { start: dst_s + (r.start - src_s), end: dst_s + (r.end - src_s) }); // Map to new range

				// If there's any range leftover, modify current and/or add split
				if let Some((r1, or2)) = symdif_range(&ranges[i], &r) {
					ranges[i] = r1;
					if let Some(r2) = or2 {
						ranges.push(r2);
					}
				} else {
					ranges.remove(i);
					continue; // Skip increment
				}
			}

			i += 1;
		}

		// Per-iteration things
		_lines += 1;
		buf.clear();
	}

	ranges.extend(new_ranges.drain(..)); // Combine
	combine_ranges(&mut ranges);
	println!("Final: {ranges:?}");
	assert_eq!(coverage, range_count(&ranges[..])); // If this panics, we lost/gained seeds.

	println!("Min: {:?}", ranges[0].start);
}
