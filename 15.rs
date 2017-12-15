/* Day 15 */

use std::fs;
use std::io::Read;
const GEN: [u64; 2] = [16807, 48271];

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("15.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	input.pop();

	let mut generators: [u32; 2] = [0; 2];
	for (i, line) in input.lines().enumerate() {
		generators[i] = line.split_whitespace().last().expect("last").parse::<u32>().expect("parse");
	}

	// let generators = [65, 8921];
	let mut judge = 0;
	let mut gen1 = generators;
	for _ in 0..40000000 {
		gen1 = gennext(&gen1);
		if gencomp(&gen1) {
			judge += 1;
		}
	}
	println!("{}", judge);

	let mut gen2 = generators;
	let mut left0 = 5000000;
	let mut left1 = left0;
	judge = 0;
	let mut list0 = Vec::new();
	let mut list1 = Vec::new();
	for _ in 0.. {
		gen2 = gennext(&gen2);
		if gen2[0] << 30 == 0 {
			list0.push(gen2[0]);
			left0 -= 1;
			// println!("0: {}", gen2[0]);
		}
		if gen2[1] << 29 == 0 {
			list1.push(gen2[1]);
			left1 -= 1;
			// println!("            1: {}", gen2[1]);
		}
		if left1 <= 0 && left0 <= 0 {break;}
	}
	for (first, second) in list0.into_iter().zip(list1.into_iter()) {
		if gencomp(&[first, second]) {
			// println!("{} {}", first, second);
			judge += 1;
		}
	}

	println!("{}", judge);
}

fn gennext(&gens: &[u32; 2]) -> [u32; 2] {
	let mut newgens: [u32; 2] = [0; 2];
	for (i, gen) in gens.iter().enumerate() {
		newgens[i] = ((*gen as u64) * GEN[i] % 2147483647) as u32;
	}
	newgens
}

fn gencomp(&gens: &[u32; 2]) -> bool {
	gens[0] << 16 == gens[1] << 16
}