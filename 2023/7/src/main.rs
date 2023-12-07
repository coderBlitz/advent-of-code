use std::{
	cmp::{Ord, Ordering, PartialOrd},
	collections::{hash_map::Entry, HashMap},
	convert::From,
	env,
	fs::File,
	io::{BufReader, BufRead},
	ops::Deref,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Card(usize);
impl From<char> for Card {
	fn from(c: char) -> Self { Card(match c {
		'A' => 14,
		'K' => 13,
		'Q' => 12,
		'J' => 11,
		'T' => 10,
		'9' => 9,
		'8' => 8,
		'7' => 7,
		'6' => 6,
		'5' => 5,
		'4' => 4,
		'3' => 3,
		_ => 2,
	})}
}
impl Deref for Card {
	type Target = usize;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandKind {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
	kind: HandKind,
	cards: [Card; 5],
	cards_map: HashMap<Card, usize>
}
impl From<&[Card]> for Hand {
	fn from(cards: &[Card]) -> Self {
		let mut counts: HashMap<Card, usize> = HashMap::new();
		for &card in cards.iter() {
			match counts.entry(card) {
				Entry::Vacant(v) => _ = v.insert(1),
				Entry::Occupied(mut o) => *o.get_mut() += 1,
			}
		}

		let max = *counts.values().max().unwrap();
		let kind = match counts.keys().count() {
			5 => HandKind::HighCard,
			1 => HandKind::FiveOfAKind,
			4 => HandKind::OnePair,
			3 if max == 3 => HandKind::ThreeOfAKind,
			3 => HandKind::TwoPair,
			2 if max == 4 => HandKind::FourOfAKind,
			2 => HandKind::FullHouse,
			_ => unreachable!(),
		};

		Hand {
			kind,
			cards: (&cards[..5]).try_into().unwrap(),
			cards_map: counts,
		}
	}
}
impl PartialOrd for Hand {
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
		match self.kind.cmp(&rhs.kind) {
			o @ (Ordering::Less | Ordering::Greater) => Some(o),
			Ordering::Equal => {
				if self.cards_map == rhs.cards_map {
					return Some(Ordering::Equal)
				}

				// Compare cards individually
				Some(self.cards.iter().cmp(rhs.cards.iter()))
			},
		}
	}
}
impl Ord for Hand {
	fn cmp(&self, rhs: &Self) -> Ordering {
		self.partial_cmp(rhs).unwrap()
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
	let mut line = String::new();
	let mut _lines = 0;
	let mut plays = Vec::new();
	while let Ok(length) = reader.read_line(&mut line) {
		if length == 0 {
			break;
		}

		/* PART 1 */
		let (cards, wager) = line.trim().split_once(' ').unwrap();
		let cards: Vec<Card> = cards.chars().map(|c| Card::from(c)).collect();
		let wager: usize = wager.parse().unwrap();

		plays.push((Hand::from(&cards[..]), wager));

		//println!("{cards:?} with {wager} -> {hand:?}");

		// Per-iteration things
		_lines += 1;
		line.clear();
	}

	plays.sort_by(|v1, v2| v1.0.cmp(&v2.0));
	/*for (rank, (hand,wager)) in plays.iter().enumerate() {
		println!("{rank}: {:?} {:?}", hand.kind, hand.cards);
	}*/
	let total: usize = plays.iter().enumerate().map(|(i, v)| (i+1) * v.1).sum();
	println!("Total: {total}");
}
