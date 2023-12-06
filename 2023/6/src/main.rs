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
	_ = reader.read_line(&mut line);

	/*let times: Vec<usize> = line.strip_prefix("Time:").unwrap()
		.trim()
		.split_whitespace()
		.map(|v| v.parse().unwrap())
		.collect();*/
	let time: usize = line.strip_prefix("Time:").unwrap()
		.trim()
		.replace(' ', "")
		.parse().unwrap();
	let times = vec![time];

	line.clear();
	_ = reader.read_line(&mut line);
	
	/*let distances: Vec<usize> = line.strip_prefix("Distance:").unwrap()
		.trim()
		.split_whitespace()
		.map(|v| v.parse().unwrap())
		.collect();*/
	let distance: usize = line.strip_prefix("Distance:").unwrap()
		.trim()
		.replace(' ', "")
		.parse().unwrap();
	let distances = vec![distance];

	println!("Time: {time}");
	println!("Distance: {distance}");

	let total: usize = times.iter().zip(distances.iter())
		.map(|(&t,&d)| (1..=t).map(|i| i*(t-i)).filter(|&res| res > d).count())
		.product();
	println!("Total: {total}");
}
