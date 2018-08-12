/* Day 18 */

/*
--- Day 18: Duet ---
You discover a tablet containing some strange assembly code labeled simply
"Duet". Rather than bother the sound card with it, you decide to run the code
yourself. Unfortunately, you don't see any documentation, so you're left to
figure out what the instructions mean on your own.

It seems like the assembly is meant to operate on a set of registers that are
each named with a single letter and that can each hold a single integer. You
suppose each register should start with a value of 0.

There aren't that many instructions, so it shouldn't be hard to figure out what
they do. Here's what you determine:

snd X plays a sound with a frequency equal to the value of X.
set X Y sets register X to the value of Y.
add X Y increases register X by the value of Y.
mul X Y sets register X to the result of multiplying the value contained in
register X by the value of Y.
mod X Y sets register X to the remainder of dividing the value contained in
register X by the value of Y (that is, it sets X to the result of X modulo Y).
rcv X recovers the frequency of the last sound played, but only when the value
of X is not zero. (If it is zero, the command does nothing.)
jgz X Y jumps with an offset of the value of Y, but only if the value of X is
greater than zero. (An offset of 2 skips the next instruction, an offset of -1
jumps to the previous instruction, and so on.)
Many of the instructions can take either a register (a single letter) or a
number. The value of a register is the integer it contains; the value of a
number is that number.

After each jump instruction, the program continues with the instruction to which
the jump jumped. After any other instruction, the program continues with the
next instruction. Continuing (or jumping) off either end of the program
terminates it.

For example:

set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
The first four instructions set a to 1, add 2 to it, square it, and then set it
to itself modulo 5, resulting in a value of 4.
Then, a sound with frequency 4 (the value of a) is played.
After that, a is set to 0, causing the subsequent rcv and jgz instructions to
both be skipped (rcv because a is 0, and jgz because a is not greater than 0).
Finally, a is set to 1, causing the next jgz instruction to activate, jumping
back two instructions to another jump, which jumps again to the rcv, which
ultimately triggers the recover operation.
At the time the recover operation is executed, the frequency of the last sound
played is 4.

What is the value of the recovered frequency (the value of the most recently
played sound) the first time a rcv instruction is executed with a non-zero
value?

Your puzzle answer was 3423.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
As you congratulate yourself for a job well done, you notice that the
documentation has been on the back of the tablet this entire time. While you
actually got most of the instructions correct, there are a few key differences.
This assembly code isn't about sound at all - it's meant to be run twice at the
same time.

Each running copy of the program has its own set of registers and follows the
code independently - in fact, the programs don't even necessarily run at the
same speed. To coordinate, they use the send (snd) and receive (rcv)
instructions:

snd X sends the value of X to the other program. These values wait in a queue
until that program is ready to receive them. Each program has its own message
queue, so a program can never receive a message it sent.
rcv X receives the next value and stores it in register X. If no values are in
the queue, the program waits for a value to be sent to it. Programs do not
continue to the next instruction until they have received a value. Values are
received in the order they are sent.
Each program also has its own program ID (one 0 and the other 1); the register p
should begin with this value.

For example:

snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
Both programs begin by sending three values to the other. Program 0 sends 1, 2,
0; program 1 sends 1, 2, 1. Then, each program receives a value (both 1) and
stores it in a, receives another value (both 2) and stores it in b, and then
each receives the program ID of the other program (program 0 receives 1; program
1 receives 0) and stores it in c. Each program now sees a different value in its
own copy of register c.

Finally, both programs try to rcv a fourth time, but no data is waiting for
either of them, and they reach a deadlock. When this happens, both programs
terminate.

It should be noted that it would be equally valid for the programs to run at
different speeds; for example, program 0 might have sent all three values and
then stopped at the first rcv before program 1 executed even its first
instruction.

Once both of your programs have terminated (regardless of what caused them to do
so), how many times did program 1 send a value?
*/

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