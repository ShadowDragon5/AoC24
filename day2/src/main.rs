use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn is_safe(report: &[i32]) -> bool
{
	let mut prev_delta: i32 = report[0] - report[1];

	for rs in report.windows(2)
	{
		let delta = rs[0] - rs[1];

		if delta == 0 || delta.abs() > 3 || delta.signum() != prev_delta.signum()
		{
			return false;
		}

		prev_delta = delta;
	}
	true
}

fn is_safe2(report: Vec<i32>) -> bool
{
	if is_safe(&report)
	{
		return true;
	}

	// if one entry is removed and the report becomes safe it is considered safe
	for i in 0..report.len()
	{
		let mut dampened = report.clone();
		dampened.remove(i);
		if is_safe(&dampened)
		{
			return true;
		}
	}

	false
}

fn read_input(filename: &str) -> u32
{
	let in_file = File::open(filename).unwrap();
	let br = BufReader::new(in_file);

	let mut safe_counter = 0;

	for res in br.lines()
	{
		let report: Vec<i32> = res
			.unwrap()
			.split_whitespace()
			.map(|x| -> i32 { x.parse().unwrap() })
			.collect();

		if is_safe2(report)
		{
			safe_counter += 1;
		}
	}

	safe_counter
}

fn main()
{
	let n = read_input("input.txt");
	println!("{n}")
}
