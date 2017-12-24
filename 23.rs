/* Day 23 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("23.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
}