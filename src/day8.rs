use std::collections::HashMap;

use itertools::Itertools;
use petgraph::Graph;

#[aoc_generator(day8)]
pub fn gen(input: &str) -> (Vec<char>, Vec<([char; 3], [char; 3], [char; 3])>) {
	let (directions, nodes) = input.split_once("\n\n").unwrap();
	let directions = directions.trim().chars().collect();

	let nodes = nodes.split('\n').map(|line| {
		let mut chars = line.chars().filter(|x| x.is_alphanumeric());
		(0..3).map(|_| {
			[chars.next().unwrap(), chars.next().unwrap(), chars.next().unwrap()]
		}).collect_tuple().unwrap()
	}).collect();
	
	(directions, nodes)
}

#[aoc(day8, part1)]
pub fn part1(input: &(Vec<char>, Vec<([char; 3], [char; 3], [char; 3])>)) -> usize {
	let (directions, nodes) = (input.0.as_slice(), input.1.as_slice());
	let mut map = HashMap::new();
	// add nodes
	for (src, l, r) in nodes {
		map.insert(*src, (*l, *r));
	}
	let mut node = ['A'; 3];
	let directions = directions.iter().cycle();
	let mut count = 0;
	for direction in directions {
		if node == ['Z'; 3] {
			break;
		}
		count += 1;
		let options = map.get(&node).unwrap();
		if *direction == 'L' {
			node = options.0;
		} else {
			node = options.1;
		}
	}
	count
}
use num::integer::lcm;

#[aoc(day8, part2)]
pub fn part2(input: &(Vec<char>, Vec<([char; 3], [char; 3], [char; 3])>)) -> usize {
	let (directions, nodemap) = (input.0.as_slice(), input.1.as_slice());
	let mut map = HashMap::new();
	let mut nodes = vec![];
	// add nodes
	for (src, l, r) in nodemap {
		map.insert(*src, (*l, *r));
		if src[2] == 'A' {
			nodes.push(*src);
		}
	}
	let directions = directions.iter().cycle();
	let mut counts = vec![];
	for node in nodes.iter_mut() {
		let mut count = 0usize;
		let directions = directions.clone();
		for direction in directions {
			if node[2] == 'Z' {
				break;
			}
			count += 1;
			let options = map.get(&*node).unwrap();
			if *direction == 'L' {
				*node = options.0;
			} else {
				*node = options.1;
			}
		}
		counts.push(count);
	}
	counts.iter().fold(1, |a, b| lcm(a, *b))
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"RL

	AAA = (BBB, CCC)
	BBB = (DDD, EEE)
	CCC = (ZZZ, GGG)
	DDD = (DDD, DDD)
	EEE = (EEE, EEE)
	GGG = (GGG, GGG)
	ZZZ = (ZZZ, ZZZ)"};

	const EXAMPLE2: &'static str = indoc!{"LR

	11A = (11B, XXX)
	11B = (XXX, 11Z)
	11Z = (11B, XXX)
	22A = (22B, XXX)
	22B = (22C, 22C)
	22C = (22Z, 22Z)
	22Z = (22B, 22B)
	XXX = (XXX, XXX)"};

	aoc_tests!(day 8 sample1, EXAMPLE=2; gen:part1);
	aoc_tests!(day 8 part1, puzzle=18673; gen:part1);

	aoc_tests!(day 8 sample2, EXAMPLE2=6; gen:part2);
	//aoc_tests!(day 8 part2, puzzle=0; gen:part2);
}
