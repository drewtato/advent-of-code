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
      
      // let mut image = "\
      //       .#.\n\
      //       ..#\n\
      //       ###\
      // ".to_string();
      
      let mut image = "\
            ##.\n\
            #.#\n\
            #..\
      ".to_string();
      
      let mut image: Vec<Vec<usize>> = [[62].to_vec()].to_vec();
      
      // println!("Image:\n{}", image);
      
      let mut rules = Vec::new();
      for line in input.lines() {
            let patterns = line.split(" => ");
            let mut rule = Vec::new();
            for pattern in patterns {
                  let rows = pattern.split('/');
                  let mut rowstring = String::new();
                  for row in rows {
                        rowstring += row;
                        rowstring += "\n";
                  }
                  rowstring.pop();
                  rule.push(rowstring);
            }
            let count = rule[0].matches('#').count();
            let rule_tuple = (count, rule[0].clone(), rule[1].clone());
            rules.push(rule_tuple);
      }
      
      rules.sort();
      
      let mut rule_map: Vec<Vec<usize>> = Vec::new();
      for (_, _, rule) in rules.iter() {
            let mut rule_map_row = Vec::new();
        'a: for section in into_sections(&rule) {
                  for (i, (_, other_rule, _)) in rules.iter().enumerate() {
                        if rules_match(&section, &other_rule) {
                              rule_map_row.push(i);
                              continue 'a;
                        }
                  }
                  panic!("Couldn't find match!")
            }
            rule_map.push(rule_map_row);
      }
      
      // let mut rules2 = rules.iter().take(6).collect::<Vec<_>>();
      // let mut rules3 = rules.iter().skip(6).collect::<Vec<_>>();
      
      // rules2.sort();
      // rules3.sort();
      
      // // println!("Rules:");
      // // for rule in rules2.iter() {
      // //       println!("{}\n", rule.1);
      // // }
      
      // // I need to convert the image into numbers that correspond to the rules
      // // that apply to each part, and I need to convert the rules into a mapping
      // // from one rule number to another so I can go through each iteration fast.
      
      // let mut rulemap2 = Vec::new();
      // for (_, _, rule2) in rules2.iter() {
      //       let mut rulenum = None;
      //       'a: for (i, (_, rule3, _)) in rules3.iter()
      //                                       .enumerate()
      //                                       .filter(
      //                                             |(_, (x, _, _))| 
      //                                             *x == rule2.matches('#').count()
      //                                       ) {
      //             for rotation in rotate(rule3) {
      //                   // println!("Checking if\n{}\n==\n{}", rule2, rule3);
      //                   if rule2 == rotation {
      //                         rulenum = Some(i);
      //                         break 'a;
      //                   }
      //             }
      //       }
      //       rulemap2.push(rulenum.expect("Couldn't find matching rule!"));
      // }
}

fn rotate(rule: &String) -> impl Iterator<Item = String> {
      unimplemented!();
}

fn into_sections(rule: &String) -> impl Iterator<Item = String> {
      unimplemented!();
}

fn rules_match(rule: &String, other_rule: &String) -> bool {
      // This will use rotate() somewhere
      unimplemented!();
}