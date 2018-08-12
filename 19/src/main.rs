/* Day 19 */

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("19.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
	let input = input;

	let mut diagram = Vec::new();
	for line in input.lines() {
		let mut v = Vec::new();
		v.push(' ');
		for c in line.chars() {
			v.push(c);
		}
		v.push(' ');
		diagram.push(v);
	}
	let mut v = Vec::new();
	for _ in diagram[0].iter() {
		v.push(' ');
	}
	diagram.push(v);

	let mut current = (0,0);
	for (x, c) in diagram[0].iter().enumerate() {
		if !c.is_whitespace() {
			current.1 = x;
			break;
		}
	}

	// println!("{:?}", current);

	let mut direction = 'd';   // d, u, l, or r
	let mut letters = String::new();
	let mut steps = 1;
	loop {
		match direction {
			'd' => current.0 += 1,
			'u' => current.0 -= 1,
			'l' => current.1 -= 1,
			'r' => current.1 += 1,
			_ => panic!("how"),
		}
		match diagram[current.0][current.1] {
			'-' | '|' => (),
			'+' => {
				match direction {
					'd' | 'u' => {
						if diagram[current.0][current.1 + 1] == ' ' {
							direction = 'l';
						} else {
							direction = 'r';
						}
					},
					'r' | 'l' => {
						if diagram[current.0 + 1][current.1] == ' ' {
							direction = 'u';
						} else {
							direction = 'd';
						}
					},
					_ => panic!("how"),
				}
			},
			' ' => break,
			a @ _ if a.is_alphabetic() => letters.push(a),
			a @ _ => panic!("found a {}", a),
		}
		steps += 1;
	}
	println!("{}\n{}", letters, steps);
}