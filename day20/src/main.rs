/* Day 20 */

/*
--- Day 20: Particle Swarm ---
Suddenly, the GPU contacts you, asking for help. Someone has asked it to
simulate too many particles, and it won't be able to finish them all in time to
render the next frame at this rate.

It transmits to you a buffer (your puzzle input) listing each particle in order
(starting with particle 0, then particle 1, particle 2, and so on). For each
particle, it provides the X, Y, and Z coordinates for the particle's position
(p), velocity (v), and acceleration (a), each in the format <X,Y,Z>.

Each tick, all particles are updated simultaneously. A particle's properties are
updated in the following order:

Increase the X velocity by the X acceleration.
Increase the Y velocity by the Y acceleration.
Increase the Z velocity by the Z acceleration.
Increase the X position by the X velocity.
Increase the Y position by the Y velocity.
Increase the Z position by the Z velocity.
Because of seemingly tenuous rationale involving z-buffering, the GPU would like
to know which particle will stay closest to position <0,0,0> in the long term.
Measure this using the Manhattan distance, which in this situation is simply the
sum of the absolute values of a particle's X, Y, and Z position.

For example, suppose you are only given two particles, both of which stay
entirely on the X-axis (for simplicity). Drawing the current states of particles
0 and 1 (in that order) with an adjacent a number line and diagram of current X
positions (marked in parenthesis), the following would take place:

p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>                         (0)(1)

p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>                      (1)   (0)

p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>          (1)               (0)

p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>                         (0)
At this point, particle 1 will never be closer to <0,0,0> than particle 0, and
so, in the long run, particle 0 will stay closest.

Which particle will stay closest to position <0,0,0> in the long term?

Your puzzle answer was 157.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
To simplify the problem further, the GPU would like to remove any particles that
collide. Particles collide if their positions ever exactly match. Because
particles are updated simultaneously, more than two particles can collide at the
same time and place. Once particles collide, they are removed and cannot collide
with anything else after that tick.

For example:

p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>    (0)   (1)   (2)            (3)
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>

p=<-3,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
p=<-1,0,0>, v=< 1,0,0>, a=< 0,0,0>             (0)(1)(2)      (3)
p=< 2,0,0>, v=<-1,0,0>, a=< 0,0,0>

p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>                       X (3)
p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>

------destroyed by collision------
------destroyed by collision------    -6 -5 -4 -3 -2 -1  0  1  2  3
------destroyed by collision------                      (3)
p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>
In this example, particles 0, 1, and 2 are simultaneously destroyed at the time
and place marked X. On the next tick, particle 3 passes through unharmed.

How many particles are left after all collisions are resolved?
*/

use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("day20/20.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
	
	// Create a Vec of particles, where each particle is a Vec of 
	// position, velocity, and acceleration, and each of those is
	// a Vec of the three dimensions.
	
	let mut particles: Vec<Vec<Vec<isize>>> = Vec::new();
	for line in input.lines() {
		let mut particle = Vec::new();
		for level_xyz in line.split_whitespace() {
			let xyz: String = level_xyz.chars().filter(|&x| "1234567890-,".contains(x)).collect();
			let mut dimensions = Vec::new();
			for dimension in xyz.split(',').take(3) {
				dimensions.push(dimension.parse::<isize>().unwrap());
			}
			particle.push(dimensions);
		}
		particles.push(particle);
	}
	
	// Create a simpler representation where all dimension values
	// have had their absolute values summed together.
	
	let mut simple_particles: Vec<Vec<isize>> = Vec::new();
	for (index, particle) in particles.iter().enumerate() {
		let mut simple_particle = Vec::new();
		simple_particle.push(index as isize);
		for level in particle.iter() {
			let mut combined = 0;
			for dimension in level.iter() {
				combined += dimension.abs();
			}
			simple_particle.push(combined);
		}
		simple_particles.push(simple_particle);
	}
	
	// Since sort is guaranteed to keep ordering between equal
	// values, this works to get the lowest acceleration first,
	// then lowest velocity, then lowest position.
	
	simple_particles.sort_by(|x,y| x[1].cmp(&y[1]));
	simple_particles.sort_by(|x,y| x[2].cmp(&y[2]));
	simple_particles.sort_by(|x,y| x[3].cmp(&y[3]));
	
	println!("{}", simple_particles[0][0]);
	
	// Part 2
	
	let mut seen = HashMap::new();
	for _ in 0..10000 {
		remove_collisions(&mut particles, &mut seen);
		update_positions(&mut particles);
	}
	println!("{}", particles.len())
}

fn remove_collisions(particles: &mut Vec<Vec<Vec<isize>>>, seen: &mut HashMap<Vec<isize>, Vec<usize>>) {
	for (index, particle) in particles.iter().enumerate() {
		let indicies = seen.entry(particle[0].clone()).or_insert(Vec::new());
		indicies.push(index);
	}
	let mut to_be_removed = Vec::new();
	for (_, indicies) in seen.iter() {
		if indicies.len() > 1 {
			for index in indicies.iter() {
				to_be_removed.push(*index);
			}
		}
	}
	to_be_removed.sort();
	for index in to_be_removed.iter().rev() {
		particles.remove(*index);
	}
	seen.clear();
}

fn update_positions(particles: &mut Vec<Vec<Vec<isize>>>) {
	for i in 0..(particles.len()) {
		for j in 0..3 {
			particles[i][1][j] += particles[i][2][j];
			particles[i][0][j] += particles[i][1][j];
		}
	}
}
