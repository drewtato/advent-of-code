/* Day 2 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("02.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	let (mut total, mut total2) = (0, 0);

	for lines in input.lines() {
		let mut max = 0;
		let mut min = 99999;
		let mut seens: Vec<i32> = Vec::new();

		for item in lines.split_whitespace().map(|x| x.parse::<i32>().expect("to_digit")) {
			let num = item;
			if max < num {
				max = num;
			}
			if min > num {
				min = num;
			}

			for seen in seens.iter() {
				if item % seen == 0 {total2 += item / seen;}
				if seen % item == 0 {total2 += seen / item;}
			}
			seens.push(item);
		}
		total += max - min;
	}
	println!("{}\n{}", total, total2);
}