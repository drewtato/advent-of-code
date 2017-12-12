/* Day 10 */
use std::fs;
use std::io::Read;
const LEN: usize = 256;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("10.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let lengths: Vec<usize> = input.split(',').filter_map(|x| x.parse().ok()).collect();
	// let lengths: Vec<usize> = vec![3,4,6];
	// println!("{:?}", lengths);
	let mut list: Vec<_> = (0..(LEN as u32)).collect();
	let mut pos: usize = 0;

	for (skip, length) in lengths.iter().enumerate() {
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
	}
	// println!("{:?}", list);
	println!("{}", list[0] * list[1]);


	// PART 2
	// let mut input = "1,2,3\n".to_string();
	while input.chars().next().expect("first").is_whitespace() {
		input.remove(0);
	}
	while input.chars().last().expect("last").is_whitespace() {
		input.pop();
	}
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

	for num in output {
		print!("{:02x}", num);
	}
	println!("\n");
}