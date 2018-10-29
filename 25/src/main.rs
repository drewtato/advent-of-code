/* Day 25 

--- Day 25: The Halting Problem ---
Following the twisty passageways deeper and deeper into the CPU, you finally 
reach the core of the computer. Here, in the expansive central chamber, you 
find a grand apparatus that fills the entire room, suspended nanometers above 
your head.

You had always imagined CPUs to be noisy, chaotic places, bustling with 
activity. Instead, the room is quiet, motionless, and dark.

Suddenly, you and the CPU's garbage collector startle each other. "It's not 
often we get many visitors here!", he says. You inquire about the stopped 
machinery.

"It stopped milliseconds ago; not sure why. I'm a garbage collector, not a 
doctor." You ask what the machine is for.

"Programs these days, don't know their origins. That's the Turing machine! 
It's what makes the whole computer work." You try to explain that Turing 
machines are merely models of computation, but he cuts you off. "No, see, 
that's just what they want you to think. Ultimately, inside every CPU, there's 
a Turing machine driving the whole thing! Too bad this one's broken. We're 
doomed!"

You ask how you can help. "Well, unfortunately, the only way to get the 
computer running again would be to create a whole new Turing machine from 
scratch, but there's no way you can-" He notices the look on your face, gives 
you a curious glance, shrugs, and goes back to sweeping the floor.

You find the Turing machine blueprints (your puzzle input) on a tablet in a 
nearby pile of debris. Looking back up at the broken Turing machine above, 
you can start to identify its parts:

A tape which contains 0 repeated infinitely to the left and right.
A cursor, which can move left or right along the tape and read or write 
values at its current position.
A set of states, each containing rules about what to do based on the current 
value under the cursor.
Each slot on the tape has two possible values: 0 (the starting value for all 
slots) and 1. Based on whether the cursor is pointing at a 0 or a 1, the 
current state says what value to write at the current position of the cursor, 
whether to move the cursor left or right one slot, and which state to use next.

For example, suppose you found the following blueprint:

Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
Running it until the number of steps required to take the listed diagnostic 
checksum would result in the following tape configurations (with the cursor 
marked in square brackets):

... 0  0  0 [0] 0  0 ... (before any steps; about to run state A)
... 0  0  0  1 [0] 0 ... (after 1 step;     about to run state B)
... 0  0  0 [1] 1  0 ... (after 2 steps;    about to run state A)
... 0  0 [0] 0  1  0 ... (after 3 steps;    about to run state B)
... 0 [0] 1  0  1  0 ... (after 4 steps;    about to run state A)
... 0  1 [1] 0  1  0 ... (after 5 steps;    about to run state B)
... 0  1  1 [0] 1  0 ... (after 6 steps;    about to run state A)
The CPU can confirm that the Turing machine is working by taking a diagnostic 
checksum after a specific number of steps (given in the blueprint). Once the 
specified number of steps have been executed, the Turing machine should pause; 
once it does, count the number of times 1 appears on the tape. In the above 
example, the diagnostic checksum is 3.

Recreate the Turing machine and save the computer! What is the diagnostic 
checksum it produces once it's working again?
*/

use std::fs;
use std::io::Read;

static A: usize = 'A' as usize;

#[derive(Debug)]
struct Turing {
    write: u8,
    mov: isize,
    next: usize,
}

fn main() {
    let mut input = String::new();
    let mut file = fs::File::open("25.txt").unwrap();
    file.read_to_string(&mut input).expect("nu");
    while input.chars().rev().next().unwrap().is_whitespace() {
        input.pop();
    }

    // Last day, lets do this
    let mut input_iter = input.split("\r\n\r\n");
    let first = input_iter.next().unwrap();
    // println!("{}", first);
    let begin_state = first.chars().nth(15).unwrap() as usize - A;
    let diagnostic_step = first.lines().nth(1).unwrap().split_whitespace().nth(5).unwrap().parse::<usize>().unwrap();
    
    let mut machine: Vec<[Turing; 2]> = Vec::new();
    for state in input_iter {
        let description: [Turing; 2];
        let mut lines = state.lines();
        lines.next().unwrap();
        lines.next().unwrap();
        
        let write1 = lines.next().unwrap().chars().nth(22).unwrap() as u8 - '0' as u8;
        let mov1 = match lines.next().unwrap().chars().nth(27).unwrap() {
            'r' => 1,
            'l' => -1,
            _ => panic!(),
        };
        let next1 = lines.next().unwrap().chars().nth(26).unwrap() as usize - A;
        lines.next().unwrap();
        let write2 = lines.next().unwrap().chars().nth(22).unwrap() as u8 - '0' as u8;
        let mov2 = match lines.next().unwrap().chars().nth(27).unwrap() {
            'r' => 1,
            'l' => -1,
            _ => panic!(),
        };
        let next2 = lines.next().unwrap().chars().nth(26).unwrap() as usize - A;
        description = [
            Turing { write: write1, mov: mov1, next: next1 },
            Turing { write: write2, mov: mov2, next: next2 }
        ];
        machine.push(description);
    }
    // println!("{} {}", begin_state, diagnostic_step);
    // for rule in machine.iter() {
    //     println!("{:?}", rule);
    // }
    
    // big array for speeeeeed
    const TAPE_SIZE: usize = 8000;
    let mut tape = [0u8; TAPE_SIZE];
    let mut current_state = begin_state;
    let mut pos = TAPE_SIZE / 2;
    for i in 0..diagnostic_step {
        let write = machine[current_state][tape[pos] as usize].write;
        let mov = machine[current_state][tape[pos] as usize].mov;
        let next = machine[current_state][tape[pos] as usize].next;
        
        tape[pos] = write;
        let pos_i = pos as isize + mov;
        if (pos_i >= TAPE_SIZE as isize) || (pos_i < 0) {
            panic!("Left tape at step {}", i);
        }
        pos = pos_i as usize;
        current_state = next;
    }
    println!("{}", tape.iter().fold(0, |acc, x| acc + *x as usize));
}