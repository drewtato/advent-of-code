/* Day 13 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("13.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	input.pop();
	input = input.chars().filter(|x| x.is_numeric() || x.is_whitespace()).collect();

	// println!("{}", input);

	let mut list: Vec<(usize, usize)> = Vec::new();
	for lines in input.lines() {
		let mut nums = lines.split_whitespace().map(|x| x.parse().expect("parse"));
		list.push((nums.next().unwrap(), nums.next().unwrap()));
	}

	// println!("{:?}", list);
	let mut severity = 0;
	for &(depth, range) in list.iter() {
		if (depth) % (range * 2 - 2) == 0 {
			severity += range * depth;
		}
	}
	println!("{}", severity);

	'delay: for delay in 0.. {
		for &(depth, range) in list.iter() {
			if (depth + delay) % (range * 2 - 2) == 0 {
				continue 'delay;
			}
		}
		println!("{}", delay);
		break;
	}
}
