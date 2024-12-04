use std::{
	fs,
	fs::File,
	io::{BufRead, BufReader},
};

fn find(line: &str) -> usize
{
	let reverse: String = line.chars().rev().collect();
	line.matches("XMAS").count() + reverse.matches("XMAS").count()
}

fn _part1(filename: &str) -> usize
{
	let in_file = File::open(filename).unwrap();
	let br = BufReader::new(in_file);

	let mut count = 0;
	let mut diagonals: Vec<String> = vec![String::new(); 140 * 2]; // ///
	let mut diagonals2: Vec<String> = vec![String::new(); 140 * 2]; // \\\
	let mut columns: Vec<String> = vec![String::new(); 140]; // |||

	for (i, res) in br.lines().enumerate()
	{
		let line = res.unwrap();
		count += find(&line);

		for (j, c) in line.chars().enumerate()
		{
			columns[j].push(c);
			diagonals[i + j].push(c);
			diagonals2[(i as i32 - j as i32 + 140) as usize].push(c);
		}
	}

	for col in columns
	{
		count += find(&col);
	}

	for (d1, d2) in diagonals.iter().zip(diagonals2)
	{
		count += find(d1);
		count += find(&d2);
	}

	count
}

fn part2(filename: &str) -> usize
{
	let input = fs::read_to_string(filename).unwrap();

	let mut count = 0;
	let matrix: Vec<Vec<i32>> = input
		.split_whitespace()
		.map(|line| {
			line.chars()
				.map(|c| match c
				{
					'X' => -10,
					'M' => -2,
					'A' => 1,
					'S' => 2,
					_ => 0,
				})
				.collect()
		})
		.collect();

	// for i in 0..matrix.len()
	// {
	// 	for j in 0..matrix[0].len()
	// 	{
	// 		print!("{:3} ", matrix[i][j]);
	// 	}
	// 	println!();
	// }

	for i in 0..matrix.len() - 2
	{
		for j in 0..matrix[0].len() - 2
		{
			//  a 0  b
			//  0 c  0
			// -b 0 -a

			if matrix[i + 1][j + 1] == 1
				&& matrix[i][j] + matrix[i + 2][j + 2] == 0
				&& matrix[i][j + 2] + matrix[i + 2][j] == 0
			{
				count += 1;
			}
		}
	}

	count
}

fn main()
{
	// let n = part2("input.txt");
	let n = part2("input2.txt");
	println!("{n}");
}
