/* Day 21 */

/*
--- Day 21: Fractal Art ---
You find a program trying to generate some art. It uses a strange process that
involves repeatedly enhancing the detail of an image through a set of rules.

The image consists of a two-dimensional square grid of pixels that are either on
(#) or off (.). The program always begins with this pattern:

.#.
..#
###
Because the pattern is both 3 pixels wide and 3 pixels tall, it is said to have
a size of 3.

Then, the program repeats the following process:

If the size is evenly divisible by 2, break the pixels up into 2x2 squares, and
convert each 2x2 square into a 3x3 square by following the corresponding
enhancement rule.
Otherwise, the size is evenly divisible by 3; break the pixels up into 3x3
squares, and convert each 3x3 square into a 4x4 square by following the
corresponding enhancement rule.
Because each square of pixels is replaced by a larger one, the image gains
pixels and so its size increases.

The artist's book of enhancement rules is nearby (your puzzle input); however,
it seems to be missing rules. The artist explains that sometimes, one must
rotate or flip the input pattern to find a match. (Never rotate or flip the
output pattern, though.) Each pattern is written concisely: rows are listed as
single units, ordered top-down, and separated by slashes. For example, the
following rules correspond to the adjacent patterns:

../.#  =  ..
          .#

                .#.
.#./..#/###  =  ..#
                ###

                        #..#
#..#/..../#..#/.##.  =  ....
                        #..#
                        .##.
When searching for a rule to use, rotate and flip the pattern as necessary. For
example, all of the following patterns match the same rule:

.#.   .#.   #..   ###
..#   #..   #.#   ..#
###   ###   ##.   .#.
Suppose the book contained the following two rules:

../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
As before, the program begins with this pattern:

.#.
..#
###
The size of the grid (3) is not divisible by 2, but it is divisible by 3. It
divides evenly into a single square; the square matches the second rule, which
produces:

#..#
....
....
#..#
The size of this enhanced grid (4) is evenly divisible by 2, so that rule is
used. It divides evenly into four squares:

#.|.#
..|..
--+--
..|..
#.|.#
Each of these squares matches the same rule (../.# => ##./#../...), three of
which require some flipping and rotation to line up with the rule. The output
for the rule is the same in all four cases:

##.|##.
#..|#..
...|...
---+---
##.|##.
#..|#..
...|...
Finally, the squares are joined into a new grid:

##.##.
#..#..
......
##.##.
#..#..
......
Thus, after 2 iterations, the grid contains 12 pixels that are on.

How many pixels stay on after 5 iterations?
*/

use std::fs;
use std::io::Read;

fn main() {
	let mut input = String::new();
	let mut file = fs::File::open("21.txt").unwrap();
	file.read_to_string(&mut input).expect("nu");
	while input.chars().rev().next().unwrap().is_whitespace() {
		input.pop();
	}
      
      let mut img = string_to_chunk(".#./..#/###");
      // println!("{:?}", img)
      
      let mut rules = Vec::new();
      for rule in input.lines() {
            let mut chunks = rule.split(" => ");
            let first = string_to_chunk(chunks.next().unwrap());
            let second = string_to_chunk(chunks.next().unwrap());
            rules.push((first, second));
      }
      // remove mutability
      let rules = rules;
      // for rule in rules {
      //       println!("{:?}", rule);
      // }
      
      for i in 0..18 {
            if i == 5 {
                  let pixels = img.iter().flatten().fold(0, |acc, &x| acc + x as u32);
                  // Print part 1
                  println!("{}", pixels);
            }
            
            let size = img.len();
            let chunk_size = if size % 2 == 0 { 2 } else { 3 };
            let chunks = image_as_chunks(&img, &size, &chunk_size);
            // for chunk in chunks.iter() {
            //       println!("{:?}", chunk);
            // }
            let mut new_image: Vec<Vec<u8>> = Vec::new();
            for chunk_row in chunks.iter() {
                  let mut bundle: Vec<Vec<u8>> = Vec::new();
                  for _ in 0..(chunk_size + 1) {
                        bundle.push(Vec::new());
                  }
                  for chunk in chunk_row.iter() {
                        let new_chunk = replace_chunk(&chunk, &chunk_size, &rules);
                        for (i, new) in new_chunk.iter().enumerate() {
                              bundle[i].append(&mut new.clone());
                        }
                  }
                  for row in bundle.iter() {
                        new_image.push(row.clone());
                  }
            }
            img = new_image;
      }
      
      let pixels = img.iter().flatten().fold(0, |acc, &x| acc + x as u32);
      // Print part 2
      println!("{}", pixels);
}

fn replace_chunk(chunk: &Vec<Vec<u8>>, chunk_size: &usize, rules: &Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>) -> Vec<Vec<u8>> {
      let mut replacement = Vec::new();
      for (rule, output) in (*rules).iter().filter(|(a, _)| a.len() == *chunk_size) {
            if matches(&rule, &chunk) {
                  replacement = output.clone();
                  break;
            }
      }
      if replacement.len() == 0 { panic!("Did not match {:?}", chunk) }
      replacement
}

fn matches(rule: &Vec<Vec<u8>>, other: &Vec<Vec<u8>>) -> bool {
      let mut rule = rule.clone();
      let mut other = other.clone();
      let mut temp = Vec::new();
      let mut temprow = Vec::new();
      for _ in 0..2 {
            for _ in 0..4 {
                  // println!("{:?} and {:?}", rule, other);
                  if rule == other { return true; }
                  temp.clear();
                  for i in 0..(rule.len()) {
                        temprow.clear();
                        for j in 0..(rule.len()) {
                              temprow.push(rule[j][rule.len()-1-i]);
                        }
                        temp.push(temprow.clone())
                  }
                  rule = temp.clone();
            }
            other.reverse();
      }
      false
}

fn image_as_chunks(img: &Vec<Vec<u8>>, size: &usize, chunk_size: &usize) -> Vec<Vec<Vec<Vec<u8>>>> {
      let mut chunks = Vec::new();
      let mut chunk = Vec::new();
      let mut chunk_row = Vec::new();
      let mut row = Vec::new();
      let start_values = (0..*size).filter(|a| a % chunk_size == 0);
      for i in start_values.clone() {
            chunk_row.clear();
            for j in start_values.clone() {
                  chunk.clear();
                  for k in 0..*chunk_size {
                        row.clear();
                        for l in 0..*chunk_size {
                              row.push(img[i + k][j + l]);
                        }
                        chunk.push(row.clone());
                  }
                  chunk_row.push(chunk.clone());
            }
            chunks.push(chunk_row.clone());
      }
      chunks
}

fn string_to_chunk(chunk: &str) -> Vec<Vec<u8>> {
      let mut v = Vec::new();
      for row in chunk.split('/') {
            let mut r = Vec::new();
            for c in row.chars() {
                  let byte: u8 = match c {
                        '#' => 1,
                        '.' => 0,
                        a => panic!("unrecognized char '{}'", a)
                  };
                  r.push(byte);
            }
            v.push(r);
      }
      v
}