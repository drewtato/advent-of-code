/* Day 4 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("04.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let mut valids = 0;
	let mut valids_2 = 0;

	'line:
	for line in input.lines() {
		let mut list = Vec::new();
 
		for word in line.split_whitespace() {
//			let word = word.chars().collect().sort();
			for seen in list.iter() {
				if &word == seen {continue 'line}
			}
			list.push(word);
		}

		valids += 1;
	}	

	'line2:
	for line in input.lines() {
		let mut list = Vec::new();
 
		for word in line.split_whitespace() {
			let mut chars: Vec<char> = word.chars().collect();
			chars.sort();
			let word: String = chars.into_iter().collect();
			for seen in list.iter() {
				if &word == seen {continue 'line2}
			}
			list.push(word);
		}

		valids_2 += 1;
	}
	println!("{}\n{}", valids, valids_2);
}