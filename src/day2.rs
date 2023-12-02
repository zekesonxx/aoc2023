
#[derive(Debug, Clone, Copy)]
pub enum Color {
	Red,
	Green,
	Blue
}
use Color::*;

impl Color {
	pub fn from_str(input: &str) -> Option<Self> {
		match input.trim() {
			"red" => Some(Self::Red),
			"green" => Some(Self::Green),
			"blue" => Some(Self::Blue),
			_ => None
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Cubes {
	color: Color,
	count: usize
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<(usize, Vec<Vec<Cubes>>)> {
	input.split('\n').map(|line| {
		// remove the Game part
		let line = line.trim_start_matches(|x: char| !x.is_ascii_digit());
		// get the game number out
		let (gamenum, line) = line.split_once(':').unwrap();
		let gamenum = gamenum.parse::<usize>().unwrap();
		// split by separate grabs
		let grabs: Vec<Vec<Cubes>> = 
		line.split(';')
			.map(str::trim)
			.map(|grab| {
				grab.split(',')
					.map(str::trim)
					.map(|cubes| {
						let (count, color) = cubes.split_once(' ').unwrap();
						Cubes {
							count: count.parse().unwrap(),
							color: Color::from_str(color).unwrap()
						}
					}).collect()
			}).collect();
		(gamenum, grabs)
	}).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(usize, Vec<Vec<Cubes>>)]) -> usize {
	let (real_red, real_green, real_blue) = (12, 13, 14);
	let mut games = 0;
	'game: for game in input {
		for grab in &game.1 {
			for cubes in grab {
				if match cubes.color {
					Red => cubes.count > real_red,
					Green => cubes.count > real_green,
					Blue => cubes.count > real_blue
				} {
					println!("breaking on game {} on grab {grab:?}", game.0);
					println!("  due to {} {:?}", cubes.count, cubes.color);
					continue 'game;
				}
			}
		}
		// we didn't break, so it must be a valid game
		games += game.0;
	}
	games
}

#[aoc(day2, part2)]
pub fn part2(input: &[(usize, Vec<Vec<Cubes>>)]) -> usize {
	let mut games = 0;
	'game: for game in input {
		let (mut r, mut g, mut b) = (0,0,0);
		for grab in &game.1 {
			for cubes in grab {
				match cubes.color {
					Red => r = r.max(cubes.count),
					Green => g = g.max(cubes.count),
					Blue => b = b.max(cubes.count),
				}
			}
		}
		println!("game {} must've needed: {r} r, {g} g, {b} b", game.0);
		games += r*g*b;
	}
	games
}

#[cfg(test)]
mod tests {
	const EXAMPLE: &'static str = indoc!{"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
	Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
	Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
	Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
	Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

	aoc_tests!(day 2 sample1, EXAMPLE=8; gen:part1);
	//aoc_tests!(day 2 part1, puzzle=0; gen:part1);

	aoc_tests!(day 2 sample2, EXAMPLE=2286; gen:part2);
	//aoc_tests!(day 2 part2, puzzle=0; gen:part2);
}
