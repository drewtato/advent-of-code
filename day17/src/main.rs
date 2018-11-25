/* Day 17 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("day17/17.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}

	let mut pos = 0;
	let step: usize = input.parse().expect("parse");
	let mut buf: Vec<usize> = vec![0];
	buf.reserve(50_000_001);
	for i in 1..2018 {
		pos = (pos + step) % i + 1;
		buf.insert(pos, i);
	}

	println!("{}", buf[pos + 1]);

	let mut ans = 0;
	for i in 2018..50_000_001 {
		pos = (pos + step) % i + 1;
		if pos == 1 {
			ans = i;
		}
	}

	println!("{}", ans);
}
