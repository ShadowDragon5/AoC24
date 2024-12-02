use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main()
{
	let in_file = File::open("input.txt").unwrap();
	println!("{:?}", in_file);

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

	A.sort();
	B.sort();

	// sum_{a, b \in A, B} | a - b |
	let sor_diff: u32 = A.iter().zip(B).map(|(a, b)| a.abs_diff(b)).sum();
	println!("Sor: {}", sor_diff);
}
