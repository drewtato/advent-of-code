/* Day 12 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("12.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	input.pop();
	let input = input.chars().filter(|x| x.is_numeric() || x.is_whitespace()).collect::<String>();

	let mut list: Vec<Vec<usize>> = Vec::new();
	for line in input.lines() {
		let words = line.split_whitespace().filter_map(|x| x.parse().ok());
		list.push(words.skip(1).collect());
	}
	// println!("{:?}", list);

	let mut known: Vec<_> = vec![false; list.len()];
	let mut stack: Vec<usize> = Vec::new();
	let mut first = true;
	let mut groups = 0;
	while known.iter().any(|x| !x) {
		groups += 1;

		stack.push(known.iter().enumerate().find(|&(_,&x)| !x).unwrap().0);

		while stack.len() > 0 {
			let current = stack.pop().expect("waat");
			known[current] = true;
			stack.extend(list[current].iter().filter(|&&x| !known[x]).cloned());
		}
		// println!("{:?}", known);
		if first {
			let mut group = 0;
			for i in known.iter() {
				if *i {
					group += 1;
				}
			}
			println!("{}", group);
			first = false;
		}
	}
	println!("{}", groups);
}