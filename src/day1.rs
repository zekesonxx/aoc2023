#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<String> {
	input.split('\n').filter(|x| x.len() > 0).map(|x| x.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> usize {
	let mut i = 0;
	for line in input {
		let mut c = line.chars().filter(|x| x.is_ascii_digit());
		let first = c.next().unwrap();
		let last = c.last().unwrap_or(first);
		println!("{line}: {first}{last}");
		i += format!("{first}{last}").parse::<usize>().unwrap_or_default();
	}
	i
}

#[aoc(day1, part2)]
pub fn part2(input: &[String]) -> usize {
	let mut i = 0;
	for line in input {
		let mut cs = line.chars();
		let mut buffer = String::new();
		let mut numbers = vec![];
		for c in cs {
			if c.is_ascii_digit() {
				numbers.push(c.to_digit(10).unwrap() as usize);
				buffer.clear();
				continue;
			}
			// add the char to the string buffer
			buffer.push(c);
			for (i, word) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
				if buffer.find(word).is_some() {
					numbers.push(i+1);
					buffer.clear();
					buffer.push(c); // this problem is stupid
					break;
				}
			}
		}
		let first = numbers[0];
		let last = numbers.last().unwrap_or(&first);
		println!("{line}: {first}{last}");
		i += (first*10) + last;
	}
	i
}

#[cfg(test)]
mod tests {
	const EXAMPLE1: &'static str = indoc!{"1abc2
	pqr3stu8vwx
	a1b2c3d4e5f
	treb7uchet"};

	const EXAMPLE2: &'static str = indoc!{"two1nine
	eightwothree
	abcone2threexyz
	xtwone3four
	4nineeightseven2
	zoneight234
	7pqrstsixteen"};

	const EXAMPLE3: &'static str = "eighthree";

	aoc_tests!(day 1 sample1, EXAMPLE1=142; gen:part1);
	//aoc_tests!(day 1 part1, puzzle=0; gen:part1);

	aoc_tests!(day 1 sample2, EXAMPLE2=281; gen:part2);
	aoc_tests!(day 1 sample3, EXAMPLE3=83; gen:part2);
	//aoc_tests!(day 1 part2, puzzle=0; gen:part2);
}
