/* Day 11 */
use std::fs;
use std::io::Read;
use std::cmp;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("11.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	input.pop();
	let list: Vec<_> = input.split(",").collect();
	let (mut s, mut ne, mut nw): (i32, i32, i32) = (0, 0, 0);
	// println!("{:?}", list);
	let mut maxdistance = 0;
	let mut distance = 0;
	for item in list.iter() {
		match *item {
			"n" => s -= 1,
			"s" => s += 1,
			"ne" => ne += 1,
			"sw" => ne -= 1,
			"nw" => nw += 1,
			"se" => nw -= 1,
			_ => (),
		}
		distance = (cmp::max(s, cmp::max(ne, nw)) - cmp::min(s, cmp::min(ne, nw))).abs();
		if maxdistance < distance {
			maxdistance = distance;
		}
	}
	// println!("{} {} {}", s, ne, nw);
	println!("{}\n{}", distance, maxdistance);
}