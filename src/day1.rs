
#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
	0
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
	0
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"1
	2
	3
	4
	5"};

	aoc_tests!(day 1 sample1, EXAMPLE=0; gen:part1);
	//aoc_tests!(day 1 part1, puzzle=0; gen:part1);

	aoc_tests!(day 1 sample2, EXAMPLE=0; gen:part2);
	//aoc_tests!(day 1 part2, puzzle=0; gen:part2);
}
