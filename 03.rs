/* Day 3 */

use std::fs;
use std::io::Read;
use std::collections::VecDeque;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("03.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	let input_int = input.parse::<i32>().expect("parse");

	let side_length = (input_int as f64).sqrt().trunc() as i32;
	let total = (((input_int - (side_length - 1).pow(2)) % side_length) - side_length / 2)
		.abs() + side_length / 2; 
	println!("{}", total);

	// comment this out to run for realsies
//	let input_int = 800;
	// the answer for 800 is 806

	// this prints the spiral
	let input_int = input_int as u32;
	let mut v: VecDeque<VecDeque<u32>> = VecDeque::new();
	v.push_back(VecDeque::new());
	v[0].push_back(1);
	let mut count: u32;
	let mut vlen = 1;

	'l: loop {
//		println!("{}", "enter loop");
		let mut first = true;

		// makes the right side except bottom and top
		for i in 0..vlen {
			count = v[i][vlen - 1];
			if !first {
				count += v[i - 1][vlen - 1] + v[i - 1][vlen];
			} else {first = false;}
			if vlen - 1 != i {
				count += v[i + 1][vlen - 1];
			}
		    v[i].push_back(count);
		    if count > input_int {break 'l}
//		    println!("{}", count);
		}
		first = true;

		// makes the bottom except left
		v.push_back(VecDeque::new());
		vlen += 1;
		for i in (0..vlen).rev() {
			count = v[vlen - 2][i];
			if !first {
				count += v[vlen - 2][i + 1] + v[vlen - 1][0];
			} else {first = false;}
			if 0 != i {
				count += v[vlen - 2][i - 1];
			}
			v[vlen - 1].push_front(count);
		    if count > input_int {break 'l}
//		    println!("{}", count);
		}
		first = true;

		// makes the left except top
		for i in (0..vlen).rev() {
			count = v[i][0];
			if !first {
				count += v[i + 1][0] + v[i + 1][1];
			} else {first = false;}
			if 0 != i {
				count += v[i - 1][0];
			}
			v[i].push_front(count);
		    if count > input_int {break 'l}
//		    println!("{}", count);
		}
		first = true;

		// makes the whole top
		v.push_front(VecDeque::new());
		vlen += 1;
		for i in 0..vlen {
			count = v[1][i];
			if !first {
				count += v[1][i - 1] + v[0][i - 1];
			} else {first = false;}
			if vlen - 1 != i {
				count += v[1][i + 1];
			}
			v[0].push_back(count);
		    if count > input_int {break 'l}
//		    println!("{}", count);
		}
	}
	// for line in v.iter() {
	// 	for num in line.iter() {
	// 		print!("{:5}", num);
	// 	}
	// 	print!("\n");
	// }
	println!("{}", count);
}

// fn get_count(v: VecDeque<VecDeque<u32>>, &pos: (usize,usize), &type: usize) -> u32 {

// }