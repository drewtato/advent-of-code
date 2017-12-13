/* Day 14 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("14.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	input.pop();

}