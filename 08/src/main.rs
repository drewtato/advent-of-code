/* Day 8 */

use std::fs;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Operation {
	Less,
	Greater,
	LessEqual,
	GreaterEqual,
	Equal,
	NotEqual,
}

#[derive(Debug)]
struct Instruction<'a> {
	reg: &'a str,
	amt: i32,
	dep: &'a str,
	opr: Operation,
	opnum: i32,
}

impl<'a> Instruction<'a> {
	fn test(&self, reglist: &mut HashMap<&'a str, i32>) -> bool {
		let dep = reglist.entry(self.dep).or_insert(0);
		match self.opr {
			Operation::Less => *dep < self.opnum,
			Operation::Greater => *dep > self.opnum,
			Operation::LessEqual => *dep <= self.opnum,
			Operation::GreaterEqual => *dep >= self.opnum,
			Operation::Equal => *dep == self.opnum,
			Operation::NotEqual => *dep != self.opnum,
		}
	}
}

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("08.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let mut instructions: Vec<Instruction> = Vec::new();
	for line in input.lines() {
		let mut words = line.split_whitespace();
		let reg = words.next().unwrap();
		let sign = match words.next().unwrap() {
			"inc" => 1,
			"dec" => -1,
			x => panic!("Not inc or dec, \"{}\" instead", x),
		};
		let amt = words.next().unwrap().parse::<i32>().expect("parse") * sign;
		words.next();
		let dep = words.next().unwrap();
		let opr = match words.next().unwrap() {
			"<" => Operation::Less,
			">" => Operation::Greater,
			"<=" => Operation::LessEqual,
			">=" => Operation::GreaterEqual,
			"==" => Operation::Equal,
			"!=" => Operation::NotEqual,
			x => panic!("what have I done there's a {}", x),
		};
		let opnum = words.next().unwrap().parse::<i32>().expect("parse");
		instructions.push(Instruction {
			reg: reg,
			amt: amt,
			dep: dep,
			opr: opr,
			opnum: opnum,
		});
	}

	let mut max = 0;
	let mut reglist = HashMap::new();
	for ins in instructions.iter() {
//		println!("{:?}", ins);
		if ins.test(&mut reglist) {
			let regnum = reglist.entry(ins.reg).or_insert(0);
			*regnum += ins.amt;
			if max < *regnum {
				max = *regnum;
			}
		}
	}

	println!("{}\n{}", reglist.values().max().unwrap(), max);

}