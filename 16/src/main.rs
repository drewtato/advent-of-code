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
	
	let input = input.split(',');
	
	let mut positions = (0..16).collect::<Vec<_>>();
	let mut letters = "abcdefghijklmnop".chars().collect::<Vec<_>>();
	let mut result = letters.clone();
	// println!("{:?}", positions);
	// println!("{:?}", letters);
	// println!("{:?}", input.collect::<Vec<_>>());
	
	let input_positions = input.clone().filter(|&x| x.chars().next().unwrap() != 'p');
	let input_letters = input.clone().filter(|&x| x.chars().next().unwrap() == 'p');
	
	for dance_move in input_positions {
		let mut dance_iter = dance_move.chars();
		match dance_iter.next() {
			Some('s') => {
				for _ in 0..(dance_iter.collect::<String>().parse::<usize>().unwrap()) {
					let temp = positions.pop().unwrap();
					positions.insert(0, temp);
				}
			},
			Some('x') => {
				let digits = dance_iter
					.collect::<String>()
					.split('/')
					.map(|x| x.parse::<usize>().unwrap())
					.collect::<Vec<_>>();
				positions.swap(digits[0], digits[1]);
			},
			Some(_) => panic!("bad character"),
			None => panic!("Dance had no characters"),
		}
	}
	
	// println!("{:?}", positions);
	
	for dance_move in input_letters {
		let mut swaps = Vec::new();
		swaps.push(dance_move.chars().nth(1).unwrap());
		swaps.push(dance_move.chars().nth(3).unwrap());
		let mut new_letters = Vec::new();
		for l in letters.clone() {
			let new = if l == swaps[0] {
				swaps[1]
			} else if l == swaps[1] {
				swaps [0]
			} else {
				l
			};
			new_letters.push(new);
		}
		letters = new_letters;
	}
	
	// println!("{:?}", letters);
	
	for _ in 0..1 {
		let mut new = Vec::new();
		for i in positions.iter() {
			new.push(result[*i]);
		}
		for (c, d) in letters.iter().zip("abcdefghijklmnop".chars()) {
			
		}
	}
	
	println!("{}", result.iter().collect::<String>());
}