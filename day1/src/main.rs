use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn read_lists(filename: &str) -> (Vec<u32>, Vec<u32>)
{
	let in_file = File::open(filename).unwrap();

	let br = BufReader::new(in_file);

	#[allow(non_snake_case)]
	let mut A = Vec::<u32>::new();
	#[allow(non_snake_case)]
	let mut B = Vec::<u32>::new();

	for res in br.lines()
	{
		let line = res.unwrap();
		let [a, b]: [&str; 2] = line
			.split_whitespace()
			.take(2)
			.collect::<Vec<&str>>()
			.try_into()
			.unwrap();

		A.push(a.parse().unwrap());
		B.push(b.parse().unwrap());
	}
	(A, B)
}

fn main()
{
	#[allow(non_snake_case)]
	let (mut A, mut B) = read_lists("input.txt");
	A.sort();
	B.sort();

	// Part 1
	// sum_{a, b \in A, B} | a - b |
	let dist: u32 = A.iter().zip(B.clone()).map(|(a, b)| a.abs_diff(b)).sum();
	println!("Part 1: {}", dist);

	// Part 2
	let counts = A
		.iter()
		.map(|&a| B.iter().filter(|&&b| a == b).count())
		.collect::<Vec<usize>>();
	let sum: u32 = A.iter().zip(counts).map(|(&x, c)| x * c as u32).sum();
	println!("Part 2: {:?}", sum);
}
