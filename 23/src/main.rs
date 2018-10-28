/* Day 23 */

/*
--- Day 23: Coprocessor Conflagration ---
You decide to head directly to the CPU and fix the printer from there. As you
get close, you find an experimental coprocessor doing so much work that the
local programs are afraid it will halt and catch fire. This would cause serious
issues for the rest of the computer, so you head in and see what you can do.

The code it's running seems to be a variant of the kind you saw recently on that
tablet. The general functionality seems very similar, but some of the
instructions are different:

set X Y sets register X to the value of Y.
sub X Y decreases register X by the value of Y.
mul X Y sets register X to the result of multiplying the value contained in
register X by the value of Y.
jnz X Y jumps with an offset of the value of Y, but only if the value of X is
not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to
the previous instruction, and so on.)
Only the instructions listed above are used. The eight registers here, named a
through h, all start at 0.

The coprocessor is currently set to some kind of debug mode, which allows for
testing, but prevents it from doing any meaningful work.

If you run the program (your puzzle input), how many times is the mul
instruction invoked?
*/

use std::fs;
use std::io::Read;

static A: usize = 'a' as usize;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("23.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
	
	let mut instructions: Vec<[&str; 3]> = Vec::new();
	for line in input.lines() {
		let mut ins = [""; 3];
		for (i, part) in line.split_whitespace().enumerate() {
			ins[i] = part;
		}
		instructions.push(ins);
	}
	// for ins in instructions.iter() {
	// 	println!("{:?}", ins);
	// }
	
	// Part 1
	let mut pc: isize = 0;
	let mut regs: [isize; 8] = [0; 8];
	let mut mul_executed = 0;
	while pc < instructions.len() as isize {
		let ins = instructions[pc as usize];
		// println!("{} {:?}", pc, ins);
		// println!("{:?}", regs);
		match ins[0] {
			"set" => {
				let first = ins[1].chars().next().unwrap();
				let index = first as usize - A;
				regs[index] = deref(ins[2], &regs);
			},
			"sub" => {
				let first = ins[1].chars().next().unwrap();
				let index = first as usize - A;
				regs[index] -= deref(ins[2], &regs);
			},
			"mul" => {
				mul_executed += 1;
				let first = ins[1].chars().next().unwrap();
				let index = first as usize - A;
				regs[index] *= deref(ins[2], &regs);
			},
			"jnz" => {
				if deref(ins[1], &regs) != 0 {
					pc += deref(ins[2], &regs) - 1;
				}
			},
			_ => panic!(),
		}
		pc += 1;
	}
	println!("{}", mul_executed);
	
	// Part 2
	// It seems like it counts the non-prime numbers that 
	// are 109900 plus a multiple of 17 and less than 126900,
	// but the answer I get is not correct so idk.
	// HA JK IT WAS AN OFF-BY-ONE coding is fun heheheheeeee*dies*
	let mut h = 0;
	for b in (109900..126901).step_by(17) {
		// println!("{}", x);
		'a: for d in 2.. {
			if d * 2 > b { break; }
			for e in 2.. {
				let prod = d * e;
				if prod > b { break; } 
				if prod == b {
					h += 1;
					break 'a;
				}
			}
		}
	}
	println!("{}", h);
}

fn deref(input: &str, regs: &[isize; 8]) -> isize {
	let first = input.chars().next().unwrap();
	if first.is_alphabetic() {
		let index = first as usize - A;
		regs[index]
	} else {
		input.parse::<isize>().unwrap()
	}
}