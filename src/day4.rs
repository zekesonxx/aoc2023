use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
        input.split('\n')
			 .filter(|x| !x.is_empty())
			 .map(|line| {
				let (_, nums) = line.split_once(":").unwrap();
				let (win, have) = nums.split_once("|").unwrap();
				let mut win: Vec<usize> = win.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
				let mut have: Vec<usize> = have.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
				win.sort();
				have.sort();
				(win, have)
			 })
			 .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
	let mut score = 0;
	for (win, have) in input {
		let mut line_score = 0;
		for number in have {
			if win.contains(number) {
				if line_score == 0 {
					line_score = 1;
				} else {
					line_score *= 2;
				}
			}
		}
		score += line_score;
	}
	score
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
	let mut cards = vec![];
	for (i, _) in input.iter().enumerate() {
		cards.push(i+1);
	}
	let mut count = 0;
	while let Some(card) = cards.pop() {
		count += 1;
		let (win, have) = &input[card-1];
		let mut line_score = 0;
		for number in have {
			if win.contains(&number) {
				line_score += 1;
			}
		}
		for i in 1..=line_score {
			cards.push(card+i);
		}
	}
	count
}

#[aoc(day4, part2, caching)]
pub fn part2_caching(input: &[(Vec<usize>, Vec<usize>)]) -> usize {
	let mut cards = vec![];
	for (i, _) in input.iter().enumerate() {
		cards.push(i+1);
	}
	let mut count = 0;
	let mut cache: HashMap<usize, Vec<usize>> = HashMap::new();
	while let Some(card) = cards.pop() {
		count += 1;
		if let Some(cached) = cache.get(&card) {
			for card in cached {
				cards.push(*card);
			}
		} else {
			let (win, have) = &input[card-1];
			let mut to_cache = vec![];
			let mut line_score = 0;
			for number in have {
				if win.contains(&number) {
					line_score += 1;
				}
			}
			for i in 1..=line_score {
				cards.push(card+i);
				to_cache.push(card+i);
			}
			cache.insert(card, to_cache);
		}
	}
	count
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
	Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
	Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
	Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
	Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
	Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"};

	aoc_tests!(day 4 sample1, EXAMPLE=13; gen:part1);
	aoc_tests!(day 4 part1, puzzle=33950; gen:part1);

	aoc_tests!(day 4 sample2, EXAMPLE=30; gen:part2, gen:part2_caching);
	aoc_tests!(day 4 part2, puzzle=14814534; gen:part2, gen:part2_caching);
}
