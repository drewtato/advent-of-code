/* Day 5 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("05.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let mut jumps = 0;
	let mut nums: Vec<i32> = input.lines().map(|x| x.parse::<i32>().expect("parse")).collect();

	let length = nums.len();
	let mut position: i32 = 0;
	loop {
		if (position < 0) | (position >= length as i32) {break}
		nums[position as usize] += 1;
		position += nums[position as usize] - 1;
	    jumps += 1;
	}

	println!("{}", jumps);

	let mut nums: Vec<i32> = input.lines().map(|x| x.parse::<i32>().expect("parse")).collect();
	jumps = 0;
	position = 0;
	loop {
		if (position < 0) | (position >= length as i32) {break}
		let offset = nums[position as usize];
		if offset < 3 {
			nums[position as usize] += 1;
		} else {
			nums[position as usize] -= 1;
		}
		position += offset;
	    jumps += 1;
	}

	println!("{}", jumps);
}