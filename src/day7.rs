use std::{collections::HashMap, str::FromStr, cmp::Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
	Ace,
	King,
	Queen,
	Jack,
	Ten,
	Nine,
	Eight,
	Seven,
	Six,
	Five,
	Four,
	Three,
	Two
}

impl Card {
	pub fn joker_cmp(&self, other: &Card) -> std::cmp::Ordering {
		if matches!(self, Card::Jack) && matches!(other, Card::Jack) {
			Ordering::Equal
		} else if matches!(self, Card::Jack) {
			Ordering::Greater
		} else if matches!(other, Card::Jack) {
			Ordering::Less
		} else {
			self.partial_cmp(other).unwrap()
		}
	}
}

impl From<char> for Card {
	fn from(value: char) -> Self {
		use Card::*;
		match value {
			'A' => Ace,
			'K' => King,
			'Q' => Queen,
			'J' => Jack,
			'T' => Ten,
			'9' => Nine,
			'8' => Eight,
			'7' => Seven,
			'6' => Six,
			'5' => Five,
			'4' => Four,
			'3' => Three,
			'2' => Two,
			_ => panic!("{value} is not a camel card")
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
	hand: [Card; 5]
}

impl FromStr for Hand {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim();
		if s.len() != 5 {
			return Err("hand is not 5 characters");
		}
		let mut iter = s.chars().map(|x| x.into());
		let hand: [Card; 5] = [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()];
		Ok(Self {
			hand
		})
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// if they don't have the same hand type, we can just order on that
		let type_cmp = self.handtype().cmp(&other.handtype());
		if !type_cmp.is_eq() {
			// if they don't have the same hand type, we can just order on that
			Some(type_cmp.reverse())
		} else {
			// they have the same hand type, so we have to compare the hands in order
			self.hand.iter()
			.zip(other.hand.iter())
			.find_map(|(mine, other)| {
				let ord = mine.partial_cmp(&other).unwrap();
				if ord.is_eq() {
					None
				} else {
					// the default derive ordering is the opposite of what we want
					Some(ord.reverse())
				}
			})
		}
	}
}

impl Hand {
	pub fn handtype(&self) -> HandType {
		use HandType::*;
		// make a hashmap of the cards and their count
		// (basically the same as doing `| sort | uniq -c`)
		let mut map: HashMap<Card, usize> = HashMap::new();
		for card in self.hand {
			*map.entry(card).or_insert(0) += 1;
		}
		let mut counts: Vec<(Card, usize)> = map.into_iter().collect();
		counts.sort_by(|(_, a), (_, b)| b.cmp(a));
		let a = counts[0].1 as usize;
		let b = counts.get(1).map(|x| x.1).unwrap_or(0);
		match (a, b) {
			(5, 0) => FiveKind,
			(4, 1) => FourKind,
			(3, 2) => FullHouse,
			(3, 1) => ThreeKind,
			(2, 2) => TwoPair,
			(2, 1) => OnePair,
			(1, 1) => HighCard,
			_ => panic!("{a}, {b}")
		}
	}
	pub fn joker(&self) -> JokerHand {
		JokerHand { hand: self.hand }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JokerHand {
	hand: [Card; 5]
}

impl JokerHand {
	pub fn handtype(&self) -> HandType {
		use HandType::*;
		// make a hashmap of the cards and their count
		// (basically the same as doing `| sort | uniq -c`)
		let mut map: HashMap<Card, usize> = HashMap::new();
		let mut jokers = 0usize;
		for card in self.hand {
			if card == Card::Jack {
				jokers += 1;
			} else {
				*map.entry(card).or_insert(0) += 1;
			}
		}
		if jokers >= 4 {
			return FiveKind;
		}
		let mut counts: Vec<(Card, usize)> = map.into_iter().collect();
		counts.sort_by(|(_, a), (_, b)| b.cmp(a));

		let a = counts[0].1 as usize;
		let b = counts.get(1).map(|x| x.1).unwrap_or(0);
		match (a, b, jokers) {
			(5, 0, 0) | (4, 0, 1) | (3, 0, 2) | (2, 0, 3) => FiveKind,
			(4, 1, 0) | (3, 1, 1) | (3, 0, 1) | (2, 1, 2) | (1, 1, 3) => FourKind,
			(3, 2, 0) | (2, 2, 1) => FullHouse,
			(3, 1, 0) | (2, 1, 1) | (1, 1, 2) => ThreeKind,
			(2, 2, 0) => TwoPair,
			(2, 1, 0) | (1, 1, 1) => OnePair,
			(1, 1, 0) => HighCard,
			_ => unreachable!("uh: {a}, {b}, {jokers}")
		}
	}
}

impl PartialOrd for JokerHand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// if they don't have the same hand type, we can just order on that
		let type_cmp = self.handtype().cmp(&other.handtype());
		if !type_cmp.is_eq() {
			// if they don't have the same hand type, we can just order on that
			Some(type_cmp.reverse())
		} else {
			// they have the same hand type, so we have to compare the hands in order
			self.hand.iter()
			.zip(other.hand.iter())
			.find_map(|(mine, other)| {
				let ord = mine.joker_cmp(&other);
				if ord.is_eq() {
					None
				} else {
					// the default derive ordering is the opposite of what we want
					Some(ord.reverse())
				}
			})
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
	FiveKind,
	FourKind,
	FullHouse,
	ThreeKind,
	TwoPair,
	OnePair,
	HighCard
}


#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<(Hand, usize)> {
	input.split('\n').map(|line| {
		let (hand, bid) = line.split_once(' ').unwrap();
		let hand = hand.parse().unwrap();
		let bid = bid.parse().unwrap();
		(hand, bid)
	}).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[(Hand, usize)]) -> usize {
	let mut hands = input.to_vec();
	hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
	hands.iter().enumerate().map(|(i, (_, bid))| bid*(i+1)).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(Hand, usize)]) -> usize {
	let mut hands: Vec<(JokerHand, usize)> = input.iter().map(|(h, b)| ((*h).joker(), *b)).collect();
	hands.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
	hands.iter().enumerate().map(|(i, (_, bid))| bid*(i+1)).sum()
}

#[cfg(test)]
mod tests {
	use std::cmp::Ordering;

	use super::*;

	#[test]
	pub fn handparsing() {
		use super::HandType::*;
		assert_eq!(FiveKind, "AAAAA".parse::<Hand>().unwrap().handtype());
		assert_eq!(FourKind, "AA8AA".parse::<Hand>().unwrap().handtype());
		assert_eq!(FullHouse, "23332".parse::<Hand>().unwrap().handtype());
		assert_eq!(ThreeKind, "TTT98".parse::<Hand>().unwrap().handtype());
		assert_eq!(TwoPair, "23432".parse::<Hand>().unwrap().handtype());
		assert_eq!(OnePair, "A23A4".parse::<Hand>().unwrap().handtype());
		assert_eq!(HighCard, "23456".parse::<Hand>().unwrap().handtype());
		let a = "33332".parse::<Hand>().unwrap();
		let b = "2AAAA".parse::<Hand>().unwrap();
		assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b));
		let a = "77888".parse::<Hand>().unwrap();
		let b = "77788".parse::<Hand>().unwrap();
		assert_eq!(Some(Ordering::Greater), a.partial_cmp(&b));
	}
	const EXAMPLE: &'static str = indoc!{"32T3K 765
	T55J5 684
	KK677 28
	KTJJT 220
	QQQJA 483"};

	aoc_tests!(day 7 sample1, EXAMPLE=6440; gen:part1);
	aoc_tests!(day 7 part1, puzzle=241344943; gen:part1);

	aoc_tests!(day 7 sample2, EXAMPLE=5905; gen:part2);
	aoc_tests!(day 7 part2, puzzle=243101568; gen:part2);
}
