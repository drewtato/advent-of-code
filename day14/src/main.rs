/* Day 14 */

use std::fs;
use std::io::Read;
const LEN: usize = 256;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("day14/14.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");


	let mut rows: Vec<String> = Vec::new();
	for i in  0..128 {
		let mut rowstring = input.clone();
		rowstring.push_str(&format!("-{}", i));
		rows.push(knothash(rowstring));
	}

	let mut mem: [[bool; 128]; 128] = [[false; 128]; 128];
	for (y, row) in rows.iter().enumerate() {
		for (x, c) in row.matches(|_| true).enumerate().map(|(x, c)| (x * 4, i8::from_str_radix(c, 16).expect("gud"))) {
			if c >= 8 {
				mem[y][x] = true;
			}
			if c % 8 >= 4 {
				mem[y][x+1] = true;
			}
			if c % 4 >= 2 {
				mem[y][x+2] = true;
			}
			if c % 2 == 1 {
				mem[y][x+3] = true;
			}
		}
	}

	let mut acc = 0;
	for r in mem.iter() {
		for &b in r.iter() {
			if b {
				acc += 1;
			}
		}
	}
	println!("{}", acc);

	let mut groups = 0;
	let mut queue: Vec<(usize, usize)> = Vec::new();
	while acc > 0 {
		if queue.is_empty() {
			groups += 1;
			'y: for y in 0..128 {
				for x in 0..128 {
					if mem[y][x] {
						mem[y][x] = false;
						acc -= 1;
						queue.push((y, x));
						break 'y;
					}
				}
			}
		}
		let mut queueadd: Vec<(usize, usize)> = Vec::new();
		for &(y, x) in queue.iter() {
			if y > 0 {
				queueadd.push((y-1, x));
			}
			if y < 127 {
				queueadd.push((y+1, x));
			}
			if x > 0 {
				queueadd.push((y, x-1));
			}
			if x < 127 {
				queueadd.push((y, x+1));
			}
		}
		queue.clear();
		for (y, x) in queueadd.into_iter() {
			if mem[y][x] {
				mem[y][x] = false;
				acc -= 1;
				queue.push((y, x));
			}
		}
	}
	println!("{}", groups);
}

fn knothash(input: String) -> String {
	let mut list: Vec<_> = (0..(LEN as u32)).collect();
	let mut lengths:Vec<_> = input.chars().map(|x| x as usize).collect();
	lengths.extend([17, 31, 73, 47, 23].iter());

	let mut skip = 0;
	let mut pos = 0;
	for _ in 0..64 {
		for length in lengths.iter() {
			// println!("{:?}", list);
			if length + pos <= LEN - 1 {
				let mut reversed: Vec<_> = list[pos..(pos + length)].iter().cloned().rev().collect();
				reversed.extend(list[(pos + length)..LEN].iter().cloned());
				list.truncate(pos);
				list.append(&mut reversed);
			} else {
				let front = (pos + length) % LEN;
				let mut reversed: Vec<_> = list.drain(pos..LEN).collect();
				reversed.extend(list.drain(0..front));
				reversed = reversed.into_iter().rev().collect();
				reversed.append(&mut list);
				let mut temp: Vec<_> = reversed.drain(0..(LEN - pos)).collect();
				reversed.append(&mut temp);
				list.append(&mut reversed);
			}
			pos = (pos + skip + length) % LEN;
			skip += 1;
		}
	}
	let mut output = Vec::new();
	for _ in 0..16 {
		output.push(list.drain(0..16).fold(0, |acc, x| acc ^ x));
	}
	let mut formatted = String::new();
	for i in output.iter() {
		formatted.push_str(&format!("{:02x}", i));
	}
	formatted
}
