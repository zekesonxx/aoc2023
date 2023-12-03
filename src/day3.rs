use std::collections::HashMap;


#[derive(Debug, Clone, Copy)]
pub enum SchematicItem {
	Number(usize),
	Symbol(char)
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> HashMap<(usize, usize), SchematicItem> {
	let mut locs: HashMap<(usize, usize), SchematicItem> = HashMap::new();
	let mut number_buffer = String::new();
	let mut last_digit_x = 0;
    for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			println!("{c}");
			if c.is_ascii_digit() {
				number_buffer.push(c);
				last_digit_x = x;
			} else if c != '.' {
				println!("adding symbol {c}");
				locs.insert((x, y), SchematicItem::Symbol(c));
			}

			if !c.is_ascii_digit() && !number_buffer.is_empty() {
				println!("adding number {number_buffer}");
				let number = number_buffer.parse::<usize>().unwrap();
				for i in 1..=number_buffer.len() {
					locs.insert((x-i, y), SchematicItem::Number(number));
				}
				number_buffer.clear();
			}
		}
		// flush the number buffer at the end of the line
		if !number_buffer.is_empty() {
			let x = line.len();
			println!("adding number {number_buffer}");
			let number = number_buffer.parse::<usize>().unwrap();
			for i in 1..=number_buffer.len() {
				locs.insert((x-i, y), SchematicItem::Number(number));
			}
			number_buffer.clear();
		}
	}
	println!("{locs:?}, {}", locs.len());
	locs
}


#[aoc(day3, part1)]
pub fn part1(input: &HashMap<(usize, usize), SchematicItem>) -> usize {
	// storing both the number and the y value (line) to dedupe across
	let mut part_numbers: Vec<(usize, usize)> = vec![];
	for ((x, y), symbol) in input.iter().filter(|(_, item)| matches!(item, SchematicItem::Symbol(_))) {
		println!("finding number(s) for ({x}, {y}) {symbol:?}");
		for x in [x.checked_sub(1).unwrap_or(*x), *x, x+1] {
			for y in [y.checked_sub(1).unwrap_or(*y), *y, y+1] {
				let Some(number) = input.get(&(x, y)) else { continue; };
				match number {
					SchematicItem::Number(number) => { 
						part_numbers.push((y, *number));
						println!("  found number {number} for {symbol:?}");
					},
					_ => {}
				}
			}
		}
	}
	part_numbers.sort();
	part_numbers.dedup();
	println!("{part_numbers:?}");
	part_numbers.iter().fold(0, |acc, (_y, num)| acc + num)
	
}

#[aoc(day3, part2)]
pub fn part2(input: &HashMap<(usize, usize), SchematicItem>) -> usize {
	0
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"467..114..
	...*......
	..35..633.
	......#...
	617*......
	.....+.58.
	..592.....
	......755.
	...$.*....
	.664.598.."};

	const EXAMPLE2: &'static str = indoc!{"467..114..
	...*......
	..35...633
	.......#..
	617*......
	.....+.58.
	..592.....
	......755.
	...$.*....
	.664.598.."};

	aoc_tests!(day 3 sample1, EXAMPLE=4361; gen:part1);
	aoc_tests!(day 3 sample2, EXAMPLE2=4361; gen:part1);
	//aoc_tests!(day 3 part1, puzzle=0; gen:part1);

	aoc_tests!(day 3 sample3, EXAMPLE=0; gen:part2);
	//aoc_tests!(day 3 part2, puzzle=0; gen:part2);
}
