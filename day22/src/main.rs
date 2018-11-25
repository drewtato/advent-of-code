/* Day 22 */

/*
--- Day 22: Sporifica Virus ---
Diagnostics indicate that the local grid computing cluster has been contaminated
with the Sporifica Virus. The grid computing cluster is a seemingly-infinite
two-dimensional grid of compute nodes. Each node is either clean or infected by
the virus.

To prevent overloading the nodes (which would render them useless to the virus)
or detection by system administrators, exactly one virus carrier moves through
the network, infecting or cleaning nodes as it moves. The virus carrier is
always located on a single node in the network (the current node) and keeps
track of the direction it is facing.

To avoid detection, the virus carrier works in bursts; in each burst, it wakes
up, does some work, and goes back to sleep. The following steps are all executed
in order one time each burst:

If the current node is infected, it turns to its right. Otherwise, it turns to
its left. (Turning is done in-place; the current node does not change.)
If the current node is clean, it becomes infected. Otherwise, it becomes
cleaned. (This is done after the node is considered for the purposes of changing
direction.)
The virus carrier moves forward one node in the direction it is facing.
Diagnostics have also provided a map of the node infection status (your puzzle
input). Clean nodes are shown as .; infected nodes are shown as #. This map only
shows the center of the grid; there are many more nodes beyond those shown, but
none of them are currently infected.

The virus carrier begins in the middle of the map facing up.

For example, suppose you are given a map like this:

..#
#..
...
Then, the middle of the infinite grid looks like this, with the virus carrier's
position marked with [ ]:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . . #[.]. . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
The virus carrier is on a clean node, so it turns left, infects the node, and
moves left:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . # . . .
. . .[#]# . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
The virus carrier is on an infected node, so it turns right, cleans the node,
and moves up:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . .[.]. # . . .
. . . . # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
Four times in a row, the virus carrier finds a clean, infects it, turns left,
and moves forward, ending in the same place and still facing up:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . #[#]. # . . .
. . # # # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
Now on the same node as before, it sees an infection, which causes it to turn
right, clean the node, and move forward:

. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
. . # .[.]# . . .
. . # # # . . . .
. . . . . . . . .
. . . . . . . . .
. . . . . . . . .
After the above actions, a total of 7 bursts of activity had taken place. Of
them, 5 bursts of activity caused an infection.

After a total of 70, the grid looks like this, with the virus carrier facing up:

. . . . . # # . .
. . . . # . . # .
. . . # . . . . #
. . # . #[.]. . #
. . # . # . . # .
. . . . . # # . .
. . . . . . . . .
. . . . . . . . .
By this time, 41 bursts of activity caused an infection (though most of those
nodes have since been cleaned).

After a total of 10000 bursts of activity, 5587 bursts will have caused an
infection.

Given your actual map, after 10000 bursts of activity, how many bursts cause a
node to become infected? (Do not count nodes that begin infected.)
*/

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("day22/22.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
	
	// make the map usable
	let mut map = Vec::new();
	for line in input.lines() {
		let mut row = Vec::new();
		for c in line.chars() {
			let node: u8 = match c {
				'#' => 2,
				'.' => 0,
				_ => panic!(),
			};
			row.push(node);
		}
		map.push(row.clone());
	}
	// for row in map.iter() {
	// 	println!("{:?}", row);
	// }
	
	// create a large grid, might need to make this bigger
	const GRID_SIZE: usize = 1000;
	let mut grid: Vec<Vec<u8>> = vec![vec![0; GRID_SIZE]; GRID_SIZE];
	let offset = GRID_SIZE / 2 - map.len() / 2;
	for (i, row) in map.iter().enumerate() {
		for (j, node) in row.iter().enumerate() {
			grid[offset + i][offset + j] = *node;
		}
	}
	// for row in grid.iter() {
	// 	println!("{:?}", row);
	// }
	
	let mut virus_pos: (usize, usize) = (GRID_SIZE / 2, GRID_SIZE / 2);
	let mut virus_direction: u8 = 0; // 0 for up, 1 for right, etc.
	let mut infections_caused: usize = 0;
	
	'a: for _ in 0..10_000_000 {
		virus_direction += grid[virus_pos.0][virus_pos.1] + 3;
		virus_direction %= 4;
		grid[virus_pos.0][virus_pos.1] += 1;
		grid[virus_pos.0][virus_pos.1] %= 4;
		if grid[virus_pos.0][virus_pos.1] == 2 {
			infections_caused += 1;
		}
		let movement: (isize, isize) = match virus_direction {
			0 => (-1, 0),
			1 => (0, 1),
			2 => (1, 0),
			3 => (0, -1),
			x => panic!("direction is {}", x),
		};
		let y = virus_pos.0 as isize + movement.0;
		let x = virus_pos.1 as isize + movement.1;
		for index in [y, x].iter() {
			if (index == &0) || (index == &(GRID_SIZE as isize)) { 
				println!("left grid at {:?}", (x, y));
				break 'a;
			}
		}
		virus_pos = (y as usize, x as usize);
	}
	
	// for row in grid.iter() {
	// 	for node in row.iter() {
	// 		let c = match node {
	// 			0 => ' ',
	// 			1 => 'W',
	// 			2 => '#',
	// 			3 => 'F',
	// 			_ => panic!(),
	// 		};
	// 		print!("{}", c)
	// 	}
	// 	println!();
	// }
	println!("{}", infections_caused);
}
