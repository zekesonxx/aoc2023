use itertools::Itertools;

#[aoc_generator(day6)]
pub fn gen(input: &str) -> (Vec<usize>, Vec<usize>) {
        input.split('\n')
		.map(|line| {
			// remove the Time: or Distance: at the beginning
			let line = line.split_once(":").unwrap().1;
			line.split_ascii_whitespace().map(|x| x.parse().unwrap_or(0)).collect()
		}).collect_tuple().unwrap()
}

#[aoc(day6, part1)]
pub fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
	let times = input.0.as_slice();
	let distances = input.1.as_slice();
	times.iter().zip(distances.iter()).map(|(race_time, record)| {
		(1..*race_time)
		.map(|button_hold_time| {
			let travel_time = *race_time-button_hold_time;
			let distance_travelled = button_hold_time*travel_time;
			distance_travelled
		})
		.filter(|distance| distance > record).count()
	}).fold(1, |a, b| a*b)
}

#[aoc(day6, part2)]
pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
	let time = input.0.iter().join("").parse::<usize>().unwrap();
	let record = input.1.iter().join("").parse::<usize>().unwrap();
	(1..time)
	.map(|button_hold_time| {
		let travel_time = time-button_hold_time;
		let distance_travelled = button_hold_time*travel_time;
		distance_travelled
	})
	.filter(|distance| *distance > record).count()
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"Time:      7  15   30
	Distance:  9  40  200"};

	aoc_tests!(day 6 sample1, EXAMPLE=288; gen:part1);
	aoc_tests!(day 6 part1, puzzle=5133600; gen:part1);

	aoc_tests!(day 6 sample2, EXAMPLE=71503; gen:part2);
	aoc_tests!(day 6 part2, puzzle=40651271; gen:part2);
}
