/* Day 6 */

use std::fs;
use std::io::Read;
const BANKCOUNT : usize = 16;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("day06/06.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let mut banks: [u32; BANKCOUNT] = [0; BANKCOUNT];
	for num in input.split_whitespace().map(|x| x.parse::<u32>().unwrap()).enumerate() {
		banks[num.0] = num.1;
	}
	let mut seens: Vec<[u32; BANKCOUNT]> = vec![banks];
	let mut index_of_match;

	'outer: loop {
		let banks2 = banks.clone();
		let mut max = (0,&0);
		for bank in banks2.iter().enumerate() {
			if bank.1 > max.1 {max = bank}
		}
		banks[max.0] = 0;

		for i in 0..*max.1 {
			banks[(max.0 + i as usize + 1) % BANKCOUNT] += 1;
		}

	    for seen in seens.iter().enumerate() {
	    	index_of_match = seen.0;
	    	if &banks == seen.1 { break 'outer }
	    }
	    seens.push(banks);
//	    println!("{:?}", banks);
	}

	println!("{}\n{}", seens.len(), seens.len() - index_of_match);
}
