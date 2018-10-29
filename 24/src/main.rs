/* Day 24 */

/*
--- Day 24: Electromagnetic Moat ---
The CPU itself is a large, black building surrounded by a bottomless pit.
Enormous metal tubes extend outward from the side of the building at regular
intervals and descend down into the void. There's no way to cross, but you need
to get inside.

No way, of course, other than building a bridge out of the magnetic components
strewn about nearby.

Each component has two ports, one on each end. The ports come in all different
types, and only matching types can be connected. You take an inventory of the
components by their port types (your puzzle input). Each port is identified by
the number of pins it uses; more pins mean a stronger connection for your
bridge. A 3/7 component, for example, has a type-3 port on one side, and a
type-7 port on the other.

Your side of the pit is metallic; a perfect surface to connect a magnetic,
zero-pin port. Because of this, the first port you use must be of type 0. It
doesn't matter what type of port you end with; your goal is just to make the
bridge as strong as possible.

The strength of a bridge is the sum of the port types in each component. For
example, if your bridge is made of components 0/3, 3/7, and 7/4, your bridge has
a strength of 0+3 + 3+7 + 7+4 = 24.

For example, suppose you had the following components:

0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
With them, you could make the following valid bridges:

0/1
0/1--10/1
0/1--10/1--9/10
0/2
0/2--2/3
0/2--2/3--3/4
0/2--2/3--3/5
0/2--2/2
0/2--2/2--2/3
0/2--2/2--2/3--3/4
0/2--2/2--2/3--3/5
(Note how, as shown by 10/1, order of ports within a component doesn't matter.
However, you may only use each port on a component once.)

Of these bridges, the strongest one is 0/1--10/1--9/10; it has a strength of
0+1 + 1+10 + 10+9 = 31.

What is the strength of the strongest bridge you can make with the components
you have available?
*/

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("24.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
	
	let mut components = Vec::new();
	for line in input.lines() {
		let mut connectors = line.split('/').map(|x| x.parse::<usize>().unwrap());
		let first = connectors.next().unwrap();
		let second = connectors.next().unwrap();
		components.push([first, second]);
	}
	// for comp in components.iter() {
	// 	println!("{:?}", comp);
	// }
	
	// Part 1
	let connection = 0;
	let (_, strength) = best_bridge(&connection, &components);
	println!("{}", strength);
	
	// Part 2
	let (_, _, strength) = longest_bridge(&connection, &components);
	println!("{}", strength);
}

fn best_bridge(connection: &usize, components: &Vec<[usize; 2]>)
	-> (Vec<[usize; 2]>, usize) {
	let mut best = Vec::new();
	let mut best_strength = 0;
	
	for (i, comp) in components.iter().enumerate().filter(|(_, &x)| (x[0] == *connection) || (x[1] == *connection)) {
		let new_connection = if comp[0] == *connection { comp[1] } else { comp[0] };
		let mut new_components = components.clone();
		let new_comp = new_components.remove(i);
		let (mut bridge, mut strength) = best_bridge(&new_connection, &new_components);
		strength += new_connection + *connection;
		if best_strength < strength {
			best = Vec::new();
			best.push(new_comp);
			best.append(&mut bridge);
			best_strength = strength;
		}
	}
	
	(best, best_strength)
}

fn longest_bridge(connection: &usize, components: &Vec<[usize; 2]>)
	-> (Vec<[usize; 2]>, usize, usize) {
	let mut best = Vec::new();
	let mut best_strength = 0;
	let mut best_length = 0;
	
	for (i, comp) in components.iter().enumerate().filter(|(_, &x)| (x[0] == *connection) || (x[1] == *connection)) {
		let new_connection = if comp[0] == *connection { comp[1] } else { comp[0] };
		let mut new_components = components.clone();
		let new_comp = new_components.remove(i);
		let (mut bridge, mut length, mut strength) = longest_bridge(&new_connection, &new_components);
		strength += new_connection + *connection;
		length += 1;
		if best_length <= length {
			if (best_length < length) || (best_strength < strength) {
				best = Vec::new();
				best.push(new_comp);
				best.append(&mut bridge);
				best_length = length;
				best_strength = strength;
			}
		}
	}
	
	(best, best_length, best_strength)
}