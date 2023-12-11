use std::{
	collections::HashMap,
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
	let mut map = Vec::new();
	let mut map_w = 0;
	let mut start = (0,0);
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		map.extend(line.trim_end().chars());
		map_w = line.trim_end().len();
		if let Some(col) = line.find('S') {
			start = (lines, col);
		}

		// Per-iteration things
		lines += 1;
		line.clear();
	}

	let start = start.0 * map_w + start.1;
	assert_eq!(map[start], 'S');

	let map_h = map.len() / map_w;
	println!("map size: {} ({map_h} x {map_w})", map.len());
	println!("Start: {start:?}");

	/* PART 1 */
	// Trace out loop
	let mut pos = start;
	let mut pipe = vec![start];

	let choose_next = |cur: usize, last: usize| -> Option<usize> {
		//println!("choosing with {cur} and {last}");
		match map[cur] {
			// If last is/was start, do full check
			'S' => {
				// North (if beyond first row)
				match map[cur.saturating_sub(map_w)] {
					'|' | '7' | 'F' if cur >= map_w && last != (cur - map_w) => return Some(cur - map_w),
					_ => (),
				};
				// East (if not in next row)
				match map[(cur + 1).min(map.len()-1)] {
					'-' | 'J' | '7' if (cur + 1) / map_w == cur / map_w && last != (cur + 1) => return Some(cur + 1),
					_ => (),
				};
				// South (if not beyond map)
				match map[(cur + map_w).min(map.len()-1)] {
					'|' | 'J' | 'L' if (cur + map_w) < map.len() && last != (cur + map_w) => return Some(cur + map_w),
					_ => (),
				};
				// Final one returns its result or none
				match map[cur.saturating_sub(1)] {
					// West (if not off map or in previous row)
					'-' | 'F' | 'L' if cur > 0 && (cur - 1) / map_w == cur / map_w && last != (cur - 1) => Some(cur - 1),
					_ => None,
				}
			},
			// North or south
			'|' => {
				// North (if beyond first row)
				match map[cur.saturating_sub(map_w)] {
					'|' | '7' | 'F' if cur >= map_w && last != (cur - map_w) => return Some(cur - map_w),
					'S' if map[last] != 'S' => None,
					_ => Some(cur + map_w),
				}
			},
			// West or east
			'-' => {
				// East (if not in next row)
				match map[(cur + 1).min(map.len()-1)] {
					'-' | 'J' | '7' if (cur + 1) / map_w == cur / map_w && last != (cur + 1) => return Some(cur + 1),
					'S' if map[last] != 'S' => None,
					_ => Some(cur - 1),
				}
			},
			// North or east
			'L' => {
				// North (if beyond first row)
				match map[cur.saturating_sub(map_w)] {
					'|' | '7' | 'F' if cur >= map_w && last != (cur - map_w) => return Some(cur - map_w),
					'S' if map[last] != 'S' => None,
					_ => Some(cur + 1),
				}
			},
			// North or west
			'J' => {
				// North (if beyond first row)
				match map[cur.saturating_sub(map_w)] {
					'|' | '7' | 'F' if cur >= map_w && last != (cur - map_w) => return Some(cur - map_w),
					'S' if map[last] != 'S' => None,
					_ => Some(cur - 1),
				}
			},
			// South or west
			'7' => {
				// South (if not beyond map)
				match map[(cur + map_w).min(map.len()-1)] {
					'|' | 'J' | 'L' if (cur + map_w) < map.len() && last != (cur + map_w) => return Some(cur + map_w),
					'S' if map[last] != 'S' => None,
					_ => Some(cur - 1),
				}
			},
			// East or south
			'F' => {
				match map[(cur + 1).min(map.len()-1)] {
					// East (if not in next row)
					'-' | 'J' | '7' if (cur + 1) / map_w == cur / map_w && last != (cur + 1) => return Some(cur + 1),
					'S' if map[last] != 'S' => None,
					_ => Some(cur + map_w),
				}
			},
			_ => None,
		}
	};

	// Choose starting next position
	pos = choose_next(pos, pos).unwrap();

	let i = 0;
	while pos != start && i < 10 {
		let last = *pipe.last().unwrap();
		pipe.push(pos);
		//println!("{} --> {}", map[last], map[pos]);

		// Find next position in following order: north, east, south, west
		// Check north (if there is a north, and north wasn't last visited)
		pos = match choose_next(pos, last) {
			Some(p) => p,
			_ => break,
		};

		//i += 1;
	}

	println!("Pipe: {pipe:?}");
	let farthest = pipe[pipe.len() / 2];
	println!("Farthest is {farthest} at {} steps", pipe.len() / 2);
	let pipe_set: HashMap<usize, usize> = pipe.iter().enumerate().map(|(i, v)| (*v, i)).collect();

	/* PART 2 */
	// Determine winding
	let mut last = pipe[1] as isize;
	let mut dir = match last - pipe[0] as isize {
		-1 => Dir::West,
		1 => Dir::East,
		x if x > 1 => Dir::South,
		x if x < -1 => Dir::North,
		_ => panic!(),
	};
	println!("Start dir: {dir:?}");
	let mut turn_count: isize = 0;
	for p in pipe.iter().skip(2) {
		let new_dir = match *p as isize - last {
			-1 => Dir::West,
			1 => Dir::East,
			x if x > 1 => Dir::South,
			x if x < -1 => Dir::North,
			_ => panic!(),
		};
		//println!("new_dir: {new_dir:?}");

		match dir {
			Dir::North => match new_dir {
				Dir::East => turn_count += 1,
				Dir::West => turn_count -= 1,
				_ => (),
			},
			Dir::East => match new_dir {
				Dir::South => turn_count += 1,
				Dir::North => turn_count -= 1,
				_ => (),
			},
			Dir::South => match new_dir {
				Dir::West => turn_count += 1,
				Dir::East => turn_count -= 1,
				_ => (),
			},
			Dir::West => match new_dir {
				Dir::North => turn_count += 1,
				Dir::South => turn_count -= 1,
				_ => (),
			},
		};

		dir = new_dir;

		last = *p as isize;
	}

	println!("Turn count: {turn_count}");
	if turn_count > 0 {
		println!("Pipe vector is clockwise!");
	} else {
		println!("Pipe vector is counter-clockwise!");
	}
}

#[derive(Debug, Eq, PartialEq)]
enum Dir {
	North,
	South,
	East,
	West,
}
