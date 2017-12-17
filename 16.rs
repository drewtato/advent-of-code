/* Day 16 */

	/*
	This is definitely the hardest one so far.
	Reddit told me to find when it loops, and then
	go from there, but for some reason I couldn't
	get it to work. The first part is commented out
	because I tried to compose all instructions of
	each type into one, but it isn't working out.
	I don't really know why.

	As of day 17, this doesn't work. This will be
	noted in the commit. This is the first one
	that I haven't been able to get.
    */

use std::fs;
use std::io::Read;
//use std::collections::VecDeque;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("16.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}

	/*let mut programs: VecDeque<char> = VecDeque::new();
	for c in ('a' as u8)..('q' as u8) {
		programs.push_back(c as char);
	}
	// println!("{:?}", programs);
	let mut first = true;
	let mut seen: Vec<Vec<char>> = Vec::new();
	loop {
		for item in input.split(',') {
			let len = item.len();
			match item.as_bytes()[0] as char {
				's' => {
					for _ in 0..item[1..len].parse::<u8>().expect("parse2") {
						let temp = programs.pop_back().unwrap();
						programs.push_front(temp);
					}
				},
				'x' => {
					let mut num = item[1..len].split('/');
					let a = num.next().unwrap().parse::<usize>().unwrap();
					let b = num.next().unwrap().parse::<usize>().unwrap();
					programs.swap(a, b);
				},
				'p' => {
					let mut c = item[1..len].chars();
					let a = c.next().unwrap();
					c.next();
					let b = c.next().unwrap();
					let ai = programs.iter().position(|&x| x == a).unwrap();
					let bi = programs.iter().position(|&x| x == b).unwrap();
					programs.swap(ai, bi);
				}
				x @ _ => panic!("umm {}", x),
			}
		}
		let progvec: Vec<char> = programs.clone().into_iter().collect();
		seen.push(progvec);
		if !first {
			if seen[0] == *seen.last().unwrap() {
				break;
			}
		}
		first = false;
	}
	for c in seen[0].iter() {
		print!("{}", c);
	}
	println!("");

	let a = 1_000_000_000 % (seen.len() - 1);
	println!("{}", seen.len());
	for c in seen[a].iter() {
		print!("{}", c);
	}
	println!(""); */

	let mut spin = 0;
	let mut swapindex: Vec<usize> = (0..16).collect();
	let mut programs = Vec::new();
	for c in ('a' as u8)..('q' as u8) {
		programs.push(c as char);
	}

	for item in input.split(',') {
		let len = item.len();
		match item.as_bytes()[0] as char {
			's' => {
				spin += item[1..len].parse::<usize>().expect("parse2");
				spin %= 16;
			},
			'x' => {
				let mut num = item[1..len].split('/');
				let a = (num.next().unwrap().parse::<usize>().unwrap() + 16 - spin) % 16;
				let b = (num.next().unwrap().parse::<usize>().unwrap() + 16 - spin) % 16;
				swapindex.swap(a, b);
			},
			'p' => (),
			x @ _ => panic!("umm {}", x),
		}
	}
	let swapindex = swapindex;
	let spin = spin;
	let mut newprog = programs.clone();
	let mut seen: Vec<Vec<char>> = Vec::new();
	loop {
		seen.push(programs.clone());
		// println!("{}\n{:?}\n{:?}", spin, swapindex, swapprog);
		for (&c, &i) in programs.iter().zip(swapindex.iter()) {
			newprog[i] = c;
		}
		for i in 0..16 {
			programs[i] = newprog[(i + spin) % 16];
		}
		if programs == seen[0] {
			break;
		}
	}
	for c in seen[1_000_000_000 % seen.len()].iter() {
		print!("{}", c);
	}
	println!("");
}