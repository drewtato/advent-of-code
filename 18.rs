/* Day 18 */

use std::fs;
use std::io::Read;
use std::collections::VecDeque;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("18.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}

	let instructions: Vec<(char, Result<char, isize>, Result<char, isize>)> = input.lines().map(|x| {
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
		let reg = {
			match reg {
				x if x.is_alphabetic() => Ok(x),
				x => Err((x as isize) - ('a' as isize)),
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
		let (reg, num) = match reg {
			Ok(x) =>((x as usize) - ('a' as usize), 0),
			Err(x) => ('z' as usize, x),
		};
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
				match (reg, num) {
					(x, 0) =>{
						if regs[x] > 0 {
							current += count - 1
						}
					},
					_ => current += count - 1,
				}
			},
			x @ _ => panic!("not a real ins: {}", x),
		}
		current += 1;
		if (current > instructions.len() as isize) || (current < 0) {break;}
	}
	println!("{}", sound);

	let mut queue1: VecDeque<isize>= VecDeque::new();
	let mut queue2: VecDeque<isize>= VecDeque::new();
	let mut current1: isize = 0;
	let mut current2: isize = 0;
	let mut reg1: Vec<isize> = vec![0; 26];
	let mut reg2: Vec<isize> = vec![0; 26];
	let mut ans = 0;
	reg2[('p' as usize) - ('a' as usize)] = 1;

	loop {
		if (current1 >= instructions.len() as isize) || (current1 < 0) {break;}
	    let (i, reg, count) = instructions[current1 as usize];
		println!("1: Begin {} at {}", i, current1);
		let (reg, num) = match reg {
			Ok(x) =>((x as usize) - ('a' as usize), 0),
			Err(x) => ('z' as usize, x),
		};
		let count = match count {
			Ok(x) => reg1[(x as usize) - ('a' as usize)],
			Err(x) => x,
		};
		match i {
			'n' => {
				queue2.push_back(count);
				ans += 1;
			},
			'e' => reg1[reg] = count,
			'd' => reg1[reg] += count,
			'u' => reg1[reg] *= count,
			'o' => reg1[reg] %= count,
			'c' => {

				// Program 2 runs

				loop {
					if (current2 >= instructions.len() as isize) || (current2 < 0) {break;}
				    let (i, reg, count) = instructions[current2 as usize];
					println!("2: Begin {} at {}", i, current2);
					let (reg, num) = match reg {
						Ok(x) =>((x as usize) - ('a' as usize), 0),
						Err(x) => ('z' as usize, x),
					};
					let count = match count {
						Ok(x) => reg2[(x as usize) - ('a' as usize)],
						Err(x) => x,
					};
					match i {
						'n' => queue1.push_back(count),
						'e' => reg2[reg] = count,
						'd' => reg2[reg] += count,
						'u' => reg2[reg] *= count,
						'o' => reg2[reg] %= count,
						'c' => {
							reg2[reg] = match queue2.pop_front() {
								Some(x) => x,
								None => break,
							}
						},
						'g' => {
							match (reg, num) {
								(x, 0) =>{
									if reg2[x] > 0 {
										current2 += count - 1
									}
								},
								_ => current2 += count - 1,
							}
						},
						x @ _ => panic!("not a real ins: {}", x),
					}
					current2 += 1;

					// Program 2 ends

				}
				reg1[reg] = match queue1.pop_front() {
					Some(x) => x,
					None => break,
				}


			},
			'g' => {
				match (reg, num) {
					(x, 0) =>{
						if reg1[x] > 0 {
							current1 += count - 1
						}
					},
					_ => current1 += count - 1,
				}
			},
			x @ _ => panic!("not a real ins: {}", x),
		}
		current1 += 1;
	}
	println!("{}", ans);
}