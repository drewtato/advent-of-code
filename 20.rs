/* Day 20 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("20.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}

	let mut particles = Vec::new();
	for line in input.lines() {
		let mut nums = line.split(|x| (x == '<') || (x == '>') || (x == ','))
								   .filter_map(|x| x.parse::<isize>().ok());
		let mut p = Vec::new();
		for _ in 0..3 {
			let mut v = Vec::new();
			for _ in 0..3 {
				v.push(nums.next().unwrap());
			}
			p.push(v);
		}
		particles.push(p);
	}

	let mut min = 500;
	let mut candidates = Vec::new();
	for (x, p) in particles.iter().enumerate() {
		let acc = p[2].iter().fold(0, |acc, &x| acc + x.abs());
		if acc < min {
			// println!("{}, {}, {:?}", acc, p[1].iter().fold(0, |acc, &x| acc + x.abs()), p);
			min = acc;
			candidates.clear();
		}
		if acc == min {
			candidates.push((x, p))
		}
	}

	let minacc = candidates;
	let mut candidates = Vec::new();
	min = 5000;
	for &(x, p) in minacc.iter() {
		// println!("{:?}", p);
		let acc = p[1].iter().fold(0, |acc, &x| acc + x.abs());
		if acc < min {
			// println!("{}, {}, {:?}", acc, p[1].iter().fold(0, |acc, &x| acc + x.abs()), p);
			min = acc;
			candidates.clear();
		}
		if acc == min {
			candidates.push((x, p))
		}
	}
	if candidates.len() == 1 {
		println!("{}", candidates[0].0);
	} else {
		println!("part 1 ehh");
	}


}