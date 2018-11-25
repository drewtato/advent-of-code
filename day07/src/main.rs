/* Day 7 */

use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("day07/07.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");

	let mut list: HashMap<&str, (u32, Vec<&str>)> = HashMap::new();

	for line in input.lines() {
		let name: &str;
		let weight: u32;
		let mut holding: Vec<&str> = Vec::new();

		let mut words = line.split_whitespace();
		name = words.next().unwrap();
		let weight_string = words.next().unwrap();
		let len_weight_string = weight_string.len();
		weight = weight_string[1..(len_weight_string - 1)].parse::<u32>().expect("parse");
		if let Some(_) = words.next() {
			for word in words {
				let len_word = word.len();
				if word[(len_word-1)..len_word] == *"," {
					holding.push(&word[0..(len_word - 1)]);
				} else {
					holding.push(word);
				}
			}
		}

		list.insert(name, (weight, holding));
	}

	// for item in list.iter() {
	// 	println!("{:?}", item);
	// }

	let mut items = list.iter();
	let mut current_item = items.next().unwrap().0;
	'l: loop {
		for item in list.iter() {
			for hold in (item.1).1.iter() {
				if current_item == hold {
					current_item = item.0;
					continue 'l;
				}
			}
		}
		break;
	}

	println!("{}", current_item);

	match weight_bad(current_item, &list) {
		Ok(_) => println!("oh no"),
		Err(x) => println!("{}", x),
	}
}

fn weight_bad<'a>(root: &str, hash: &'a HashMap<&str, (u32, Vec<&str>)>) -> Result<u32, u32> {
	let mut weight = hash.get(root).expect("not in table :O").0;
	let children = hash.get(root).expect("not in table???").1.iter();
	let mut weights = Vec::new();
	for child in children {
		weight += match weight_bad(child, hash) {
			Ok(x) => {
				weights.push((x, child));
				x
			},
			x @ Err(_) => return x,
		};
	}

	if weights.len() > 2 {
		weights.sort();
		let len = weights.len() - 1;
		if weights[0].0 != weights[len].0 {
			let mistake =
			if weights[0].0 != weights[1].0 {
				(weights[0].0 - weights[1].0, weights[0].1)
			} else {
				(weights[len].0 - weights[1].0, weights[len].1)
			};

			let supposed = hash.get(mistake.1).expect("lol").0 - mistake.0;
			return Err(supposed);
		}
	}
	Ok(weight)
}
