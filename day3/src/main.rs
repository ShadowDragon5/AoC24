use std::fs;

use regex::Regex;

fn _part1(input: String)
{
	let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
	let matches = re.captures_iter(&input);

	let mut sum: i32 = 0;
	for m in matches
	{
		let a: i32 = m[1].parse().unwrap();
		let b: i32 = m[2].parse().unwrap();
		sum += a * b;
	}
	println!("{sum}")
}

fn part2(input: String)
{
	let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
	let matches = re.captures_iter(&input);

	let mut sum: i32 = 0;
	let mut enabled = true;

	for m in matches
	{
		if &m[0] == r"do()"
		{
			enabled = true;
		}
		else if &m[0] == r"don't()"
		{
			enabled = false;
		}
		else if enabled
		{
			let a: i32 = m[1].parse().unwrap();
			let b: i32 = m[2].parse().unwrap();
			sum += a * b;
		}
	}
	println!("{sum}")
}

fn main()
{
	let input = fs::read_to_string("input.txt").unwrap();
	part2(input);
}
