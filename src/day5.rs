use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Map {
	pub ranges: Vec<(usize, usize, usize)>
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Map(\n")?;
		for (dst_start, src_start, len) in &self.ranges {
			let (dst_end, src_end) = (dst_start+len-1, src_start+len-1);
			write!(f, "  {src_start}-{src_end} to {dst_start}-{dst_end} (len {len})\n")?;
		}
		write!(f, ")")
    }
}

impl Map {
	pub fn parse(input: &str) -> Self {
		Map {
			ranges: input.split('\n').map(|line| {
				line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_tuple().unwrap()
			}).collect()
		}
	}
	pub fn map(&self, input: usize) -> usize {
		for (dst_start, src_start, len) in &self.ranges {
			if input >= *src_start && input < src_start+len {
				return dst_start+(input-src_start);
			}
		}
		input
	}
}


#[aoc_generator(day5)]
pub fn gen(input: &str) -> (Vec<usize>, Vec<Map>) {
        let mut sections: Vec<&str> = input.split("\n\n").collect();
		let seeds = sections.remove(0);
		let seeds = seeds.split_once(":").unwrap().1;
		let seeds: Vec<usize> = seeds.split_ascii_whitespace().map(|x| x.parse().unwrap_or(0)).collect();
		let maps = sections.iter().map(|section| {
			let section = section.split_once(":\n").unwrap().1; // remove the header
			Map::parse(section)
		}).collect();
		(seeds, maps)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<usize>, Vec<Map>)) -> usize {
	let seeds = input.0.as_slice();
	let maps = input.1.as_slice();
	seeds.iter().map(|seed| {
		maps.iter().fold(*seed, |i, map| map.map(i))
	}).min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<usize>, Vec<Map>)) -> usize {
	let seeds = input.0.as_slice();
	let maps = input.1.as_slice();
	seeds.par_chunks(2)
	.map(|x| {
		x[0]..(x[0]+x[1])
	})
	.flatten()
	.map(|seed| {
		maps.iter().fold(seed, |i, map| map.map(i))
	}).min().unwrap()
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn test_map() {
		use super::Map;
		let map = Map {
			ranges: vec![(50, 98, 2), (52, 50, 48)]
		};
		println!("{}", map);
		assert_eq!(map.map(79), 81);
		assert_eq!(map.map(14), 14);
		assert_eq!(map.map(55), 57);
		assert_eq!(map.map(13), 13);
		assert_eq!(map.map(97), 99);
		assert_eq!(map.map(98), 50);
		assert_eq!(map.map(99), 51);
		assert_eq!(map.map(100), 100);
	}
	const EXAMPLE: &'static str = indoc!{"seeds: 79 14 55 13

	seed-to-soil map:
	50 98 2
	52 50 48
	
	soil-to-fertilizer map:
	0 15 37
	37 52 2
	39 0 15
	
	fertilizer-to-water map:
	49 53 8
	0 11 42
	42 0 7
	57 7 4
	
	water-to-light map:
	88 18 7
	18 25 70
	
	light-to-temperature map:
	45 77 23
	81 45 19
	68 64 13
	
	temperature-to-humidity map:
	0 69 1
	1 0 69
	
	humidity-to-location map:
	60 56 37
	56 93 4"};

	aoc_tests!(day 5 sample1, EXAMPLE=35; gen:part1);
	aoc_tests!(day 5 part1, puzzle=551761867; gen:part1);

	aoc_tests!(day 5 sample2, EXAMPLE=46; gen:part2);
	aoc_tests!(day 5 part2, puzzle=57451709; gen:part2);
}
