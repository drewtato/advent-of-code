/* Day 9 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("09.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let mut score = 0;
	let mut total_score = 0;
	let mut garbage = 0;
	let mut ignore = false;
	let mut ignore_next = false;

	for c in input.chars() {
		if ignore_next {
			ignore_next = false;
		} else if ignore {
		    match c {
		    	'>' => ignore = false,
		    	'!' => ignore_next = true,
		    	_ => garbage += 1,
		    }
		} else {
			match c {
				'{' => {
						score += 1;
					},
				'}' => {
						total_score += score;
						score -= 1;
					},
				'<' => ignore = true,
				'!' => ignore_next = true,
				_ => (),
			}
		}
	}
	println!("{}\n{}", total_score, garbage);
}