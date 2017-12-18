/* Day 18 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("18.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}

	let instructions: Vec<(char, char, Result<char, isize>)> = input.lines().map(|x| {
		let ins = x.chars().nth(1).unwrap();
		let reg = x.chars().nth(4).unwrap();
		let count = x.chars().skip(6).collect::<String>();
		let a = count.chars().next();
		let count = {
			match count.len() {
				0 => Err(0),
				1 if a.unwrap().is_alphabetic() => Ok(a.unwrap()),
				_ => Err(count.parse::<isize>().unwrap()),
			}
		};
		(ins, reg, count)
	} ).collect();

	// println!("{:?}", instructions);

	let mut regs: Vec<isize> = vec![0; 26]; // way unnecessary but works
	let mut sound: isize = 0;
	let mut current: isize = 0;

	loop {
		let (i, reg, count) = instructions[current as usize];
		let reg = (reg as usize) - ('a' as usize);
		let count = match count {
			Ok(x) => regs[(x as usize) - ('a' as usize)],
			Err(x) => x,
		};
		match i {
			'n' => sound = regs[reg],
			'e' => regs[reg] = count,
			'd' => regs[reg] += count,
			'u' => regs[reg] *= count,
			'o' => regs[reg] %= count,
			'c' => {
				if regs[reg] != 0 {break;}
			},
			'g' => {
				if regs[reg] > 0 {
					current += count - 1;
				}
			},
			x @ _ => panic!("not a real ins: {}", x),
		}
		current += 1;
		if (current > instructions.len() as isize) || (current < 0) {break;}
	}
	println!("{}", sound);
}