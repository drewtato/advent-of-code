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
      
      // let mut image = "\
      //       ##.\n\
      //       #.#\n\
      //       #..\
      // ".to_string();
      
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
                  }
                  rule.push(rowstring);
            }
            let count = rule[0].matches('#').count();
            let rule_tuple = (count, rule[0].clone(), rule[1].clone());
            rules.push(rule_tuple);
      }
      
      rules.sort();
      
      // This big guy is a list of rule numbers.
      let mut rule_map: Vec<Vec<usize>> = Vec::new();
      // Each value is either a single number or four numbers, for when
      // there's a 3x3 result or a 4x4 result respectively. The number 
      // corresponds to the index of the rule it applies to in the
      // rules vec. The 4x4 results become four different 2x2 rules. 
      
      for (_, _, rule) in rules.iter() {
            let mut rule_map_row = Vec::new();
            let mut sections = Vec::new();
            into_sections(&rule, &mut sections);
        'a: for section in sections.iter() {
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
      
      // for (i, (line, (_, rule, other))) in rule_map.iter().zip(rules.iter()).enumerate() {
      //       print!("{}: {:?} ", i, line);
      //       println!("{}, {}", rule, other);
      // }
      
      let mut image: Vec<Vec<usize>> = [[62].to_vec()].to_vec();
      
      for i in 0..5 {
            let mut temp = Vec::new();
            if i % 2 != 0 {
                  for row in image.iter() {
                        let mut top = Vec::new();
                        for group in row.iter() {
                              top.push(rule_map[*group][0]);
                        }
                        temp.push(top);
                  }
            } else {
                  for row in image.iter() {
                        let mut top = Vec::new();
                        let mut bottom = Vec::new();
                        for group in row.iter() {
                              top.push(rule_map[*group][0]);
                              top.push(rule_map[*group][1]);
                              bottom.push(rule_map[*group][2]);
                              bottom.push(rule_map[*group][3]);
                        }
                        temp.push(top);
                        temp.push(bottom);
                  }
            }
            image = temp;
      }
      
      let count = image.iter().fold(0, |acc, x| acc + x.iter().fold(0, |acc, y| acc + rules[*y].0));
      println!("{}", count);
}

fn rotate(rule: &mut String) {
      let mut temp = String::new();
      let v = match rule.chars().count() {
            9 => [3,6,9,2,5,8,1,4,7].to_vec(),
            4 => [2,4,1,3].to_vec(),
            _ => panic!("bad rule count: {}", rule),
      };
      for index in v.iter() {
            temp.push(rule.chars().nth(*index - 1).expect("Rotate failed"));
      }
      *rule = temp;
}

fn into_sections(rule: &String, sections: &mut Vec<String>) {
      match rule.chars().count() {
            9 => sections.push(rule.clone()),
            16 => {
                  for i in 0..2 {
                        for j in 0..2 {
                              let mut section = String::new();
                              for k in [0, 1, 4, 5].iter() {
                                    section.push(rule.chars().nth((k + 2*j + 8*i) as usize).expect("Sections failed"));
                              }
                              sections.push(section);
                        }
                  }
            },
            _ => panic!("Not a valid rule. Here's ur terrible rule: {}", rule)
      };
}

fn rules_match(rule: &String, other_rule: &String) -> bool {
      let original = other_rule.clone();
      let mut flipped = String::new();
      let v = match original.chars().count() {
            9 => [3,2,1,6,5,4,9,8,7].to_vec(),
            4 => [2,1,4,3].to_vec(),
            _ => panic!("bad rule count: {}", original),
      };
      for index in v.iter() {
            flipped.push(original.chars().nth(*index - 1).expect(&format!("\nMatch failed:\n{}\n{}\n{}\n", rule, other_rule, index)))
      }
      for flip in &mut [original, flipped] {
            for _ in 0..4 {
                  if *rule == *flip {
                        return true;
                  }
                  rotate(flip);
            }
      }
      false
}