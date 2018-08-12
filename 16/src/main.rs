/* Day 16 */

/*
--- Day 16: Permutation Promenade ---
You come upon a very unusual sight; a group of programs here appear to be
dancing.

There are sixteen programs in total, named a through p. They start by standing
in a line: a stands in position 0, b stands in position 1, and so on until p,
which stands in position 15.

The programs' dance consists of a sequence of dance moves:

Spin, written sX, makes X programs move from the end to the front, but maintain
their order otherwise. (For example, s3 on abcde produces cdeab).
Exchange, written xA/B, makes the programs at positions A and B swap places.
Partner, written pA/B, makes the programs named A and B swap places.
For example, with only five programs standing in a line (abcde), they could do
the following dance:

s1, a spin of size 1: eabcd.
x3/4, swapping the last two programs: eabdc.
pe/b, swapping programs e and b: baedc.
After finishing their dance, the programs end up in order baedc.

You watch the dance for a while and record their dance moves (your puzzle
input). In what order are the programs standing after their dance?

Your puzzle answer was fgmobeaijhdpkcln.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
Now that you're starting to get a feel for the dance moves, you turn your
attention to the dance as a whole.

Keeping the positions they ended up in from their previous dance, the programs
perform it again and again: including the first dance, a total of one billion
(1000000000) times.

In the example above, their second dance would begin with the order baedc, and
use the same dance moves:

s1, a spin of size 1: cbaed.
x3/4, swapping the last two programs: cbade.
pe/b, swapping programs e and b: ceadb.
In what order are the programs standing after their billion dances?
*/
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