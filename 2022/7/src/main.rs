use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{BTreeMap};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, PartialEq)]
enum DirEntryKind {
	File,
	Dir,
}

#[derive(PartialEq, Clone)]
struct DirEntry {
	name: String,
	size: usize,
	kind: DirEntryKind
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
	let files = &mut BTreeMap::<String, DirEntry>::new();
	let mut cwd = PathBuf::new();
	let mut line = String::new();
	let mut _lines = 0;
	let mut collecting_files = false;
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}
		let trimmed = line.trim();

		/*
		Parse command to determine action for next line(s)
		If cd
			change cwd
		If ls
			Put all entries in treemap, and map it to the Vector
		*/
		if &trimmed[0..1] == "$" {
			// Finish last collection command
			collecting_files = false;

			// CMD stuff
			let command: Vec<&str> = trimmed[2..].split_whitespace().collect();
			match command[0] {
				"cd" => {
					//println!("moving to {}!", command[1]);
					if command[1] == ".." {
						cwd.pop();
					} else {
						cwd.push(command[1]);
					}
				},
				"ls" => {
					collecting_files = true;
					//println!("listing!");
				},
				_ => panic!("Unknown!"),
			};
		} else if collecting_files {
			// Parse file entry
			let entry: Vec<&str> = trimmed.split_whitespace().collect();

			// Determine kind
			let kind;
			let size;
			if let Ok(sz) = entry[0].parse() {
				kind = DirEntryKind::File;
				size = sz;
			} else {
				kind = DirEntryKind::Dir;
				size = 0;
			}

			// Push entry
			let fname = entry[1].to_string();
			let path = &mut PathBuf::new();
			path.push(&cwd);
			path.push(&fname);
			files.insert(path.to_str().unwrap().to_string(), DirEntry {
				name: fname,
				size: size,
				kind: kind
			});
		}

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	println!("Parsed {} files!", files.len());

	// Insert root folder for total size
	let root = String::from("/");
	files.insert(root.clone(), DirEntry {
		name: root,
		size: 0,
		kind: DirEntryKind::Dir
	});

	// Collect all keys, sort by longest paths first
	let mut keys: Vec<_> = files.keys().cloned().collect();
	keys.sort_by_cached_key(|x| x.matches('/').count());
	keys.reverse();

	// Loop over files and add sizes to parent directory
	for file in &keys {
		let ent = files.get(file).unwrap();
		let ent_size = ent.size;

		if let Some(par) = Path::new(&file).parent() {
			if let Some(par_ent) = files.get_mut(&par.to_str().unwrap().to_string()) {
				par_ent.size += ent_size;
			} else {
				println!("parentless! = {file}");
			}
		}
	}

	/* Part 1
	*/
	let mut sum = 0;
	let dirs = &mut Vec::new();
	for (path, entry) in files.iter() {
		if entry.kind == DirEntryKind::Dir {
			//println!("{} has size {}", path, entry.size);
			if entry.size <= 100_000 {
				sum += entry.size;
			}

			dirs.push((path, entry));
		}
	}

	println!("Under 100k total = {sum}");

	/* Part 2
	*/
	let all_sizes: &mut Vec<(_,_)> = &mut files.iter().collect();
	all_sizes.sort_by_key(|x| x.1.size);

	// Have 70 mil space, need at least 30 mil free. So need root to be less than 40 mil. Take difference
	let rt = String::from("/");
	let target = files.get(&rt).unwrap().size - 40_000_000;
	println!("Target = {target}");

	for (path, dir) in all_sizes {
		if dir.size >= target {
			println!("{path} with size _{}", dir.size);
			break;
		}
	}
}
