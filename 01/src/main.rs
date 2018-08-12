/* Day 1 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("01.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	let first = input.chars().next().expect("nuu").to_digit(10).expect("nuuu");
	let mut last = 0;
	let mut total = 0;
//	println!("first: {}\nlast: {}", first, last);
	for n in input.chars() {
		if n == '\n' {break}
		total += if n.to_digit(10).expect("nuuu2") == last {last} else {0};
		last = n.to_digit(10).expect("nuuu3");
//		println!("total: {}\nlast: {}", total, last);
	}
	total += if first == last {last} else {0};
	println!("{}", total);
	let mut total2 = 0;
	let numvec: Vec<usize> = input.chars()
								 .map(|x| x.to_digit(10))
								 .filter(|x| x.is_some())
								 .map(|x| x.unwrap() as usize)
								 .collect();
	let half = numvec.len() / 2;
	for i in 0..half {
		if numvec[i] == numvec[i+half] {
			total2 += numvec[i] * 2;
		}
	}

	println!("{}", total2);
}