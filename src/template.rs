
#[aoc_generator(dayDAYNUMBER)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
}

#[aoc(dayDAYNUMBER, part1)]
pub fn part1(input: &[usize]) -> usize {
	0
}

#[aoc(dayDAYNUMBER, part2)]
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

	aoc_tests!(day DAYNUMBER sample1, EXAMPLE=0; gen:part1);
	//aoc_tests!(day DAYNUMBER part1, puzzle=0; gen:part1);

	aoc_tests!(day DAYNUMBER sample2, EXAMPLE=0; gen:part2);
	//aoc_tests!(day DAYNUMBER part2, puzzle=0; gen:part2);
}
