/// Macro to automatically generate tests for AOC solver functions
/// 
/// 
/// Format:
/// ```text
/// aoc_tests!(day # <module name>, <example input>=<result>; <gen>:<calc>(,<gen>:<calc>,...))
/// ```
/// where:
/// * `#` is the day number, from 1-25
/// * `<module name>` is the name of the test, usually `sample1`, `sample2`, `part1`, or `part2`
/// * `<example input>` is the name of a constant variable holding sample input for your code
/// * `<result>` is the expected result from the given input
/// * `<gen>:<calc>` is one or more functions to test
///   * `<gen>` is your generator function, these can be reused for multiple calculation functions
///   * `<calc1>` is your calculator function, these must be uniquely named
/// 
/// The macro expects to be run within a `mod tests {}` in the day's module.  
/// The module will try to find the example input constant in the current context,
/// and the functions in the context one module up (imagine them prefixed with `super::`).
/// 
/// There's also a special case for testing against the actual puzzle input:
/// ```text
/// aoc_tests!(day # part<1|2>, puzzle=<result>; <gen>:<calc>(,<gen>:<calc>,...))
/// ```
/// where the module name is `part1` or `part2`, and `<result>` is the known correct answer to the puzzle
/// 
/// Example use:
// this is no_run because we can't make doctests run literal tests
// and because this macro generates tests, we can't actually test them here
/// ```no_run
/// # #[macro_use] extern crate aoc2022;
/// pub fn gen(input: &[usize]) -> Vec<usize> {
///     input.to_owned()
/// }
/// 
/// pub fn part1(input: &[usize]) -> usize {
///    input.iter().sum() 
/// }
/// 
/// pub fn part1_alternate(input: &[usize]) -> usize {
///   let mut c = 0;
///   for i in input {
///     c += i;
///   }
///   c
/// }
/// 
/// #[cfg(test)]
/// mod tests {
///     // test against the example given to us by aoc
///     const EXAMPLE: &[usize] = &[1,2,3,4,5];
///     aoc_tests!(day 1 sample1, EXAMPLE=15; gen:part1, gen:part1_alternate);
/// 
///     // test against the real puzzle
///     aoc_tests!(day 1 part1, puzzle=1337; gen:part1, gen:part1_alternate);
/// }
/// 
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! aoc_tests {
	(day $day:literal part1, puzzle=$puzzle_answer:literal; $($gen_func:ident:$part_func:ident),+) => {
		aoc_tests!(____puzzle puzzle1 $day $puzzle_answer $($gen_func:$part_func),+);
	};
	(day $day:literal part2, puzzle=$puzzle_answer:literal; $($gen_func:ident:$part_func:ident),+) => {
		aoc_tests!(____puzzle puzzle2 $day $puzzle_answer $($gen_func:$part_func),+);
	};
	(____puzzle $ident:ident $day:literal $puzzle_answer:literal $($gen_func:ident:$part_func:ident),+) => {
		#[cfg(test)]
		mod $ident {
			lazy_static! {
				static ref INPUT: &'static str = {
					include_str!(concat!("../input/2022/day", $day, ".txt")).trim_end_matches('\n')
				};
			}
			$(
				#[test]
				fn $part_func() {
					assert_eq!(super::super::$part_func(&super::super::$gen_func(&INPUT)), $puzzle_answer);
				}
			)+
		}
	};
	(day $day:literal $section:ident, $sample_input:ident=$sample_result:literal; $($gen_func:ident:$part_func:ident),+) => {
		#[cfg(test)]
		mod $section {
			$(
				#[test]
				fn $part_func() {
					assert_eq!(super::super::$part_func(&super::super::$gen_func(super::$sample_input)), $sample_result);
				}
			)+
		}
	};
}