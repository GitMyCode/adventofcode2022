/*
--- Day 5: Supply Stacks ---
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?


--- Part Two ---
As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3
Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3
Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

use std::{fs::File, io::Read, usize};

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut fileContent = String::new();
    file.read_to_string(&mut fileContent);
    return fileContent;
}

fn parse_stack(content: &str) -> (Vec<&str>, i32) {
    let mut all_stack: Vec<Vec<&str>> = Vec::new();
    let split: Vec<&str> = content.split("\n\n").collect();
    let mut stack_input_iterator = split[0].lines().enumerate().peekable();
    let mut total_stack = 0;
    let mut stack_value_input: Vec<&str> = Vec::new();
    while let Some((index, line)) = stack_input_iterator.next() {
        // last line
        if (stack_input_iterator.peek().is_none()) {
            total_stack = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        } else {
            stack_value_input.push(line);
        }
    }

    return (stack_value_input, total_stack);
}
fn make_stack_vectors<'a>(stack_values: Vec<&'a str>, total_stack: &i32) -> Vec<VecDeque<&'a str>> {
    let mut all_stack: Vec<VecDeque<&'a str>> = Vec::new();

    // [R] [T] [T] [R] [G] [W] [F] [W] [L]
    // 0123456789
    //
    for stack_number in 0..*total_stack {
        let mut stack: VecDeque<&'a str> = VecDeque::new();
        let start_char_index = if stack_number == 0 {
            (3 * stack_number)
        } else {
            (3 * stack_number) + stack_number
        };
        println!("\t{}", stack_number);
        let mut crate_string: &'a str;
        for crate_value in &stack_values {
            let split_to = (start_char_index + 3) as usize;
            crate_string = &crate_value[start_char_index as usize..split_to];
            if (!crate_string.trim().is_empty()) {
                stack.push_back(crate_string);
            }
        }
        all_stack.push(stack);
    }
    return all_stack;
}
// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicite lifetime

struct Move {
    from: i32,
    to: i32,
    count: i32,
}

fn parse_moves(content: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let moves_content: &str = content.split("\n\n").collect::<Vec<&str>>()[1];
    let moves_lines: Vec<&str> = moves_content.lines().collect();
    for line in moves_lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Extract the first, third, and fifth tokens (which should be the integers)
        let first: i32 = tokens[1].parse().unwrap();
        let third: i32 = tokens[3].parse().unwrap();
        let fifth: i32 = tokens[5].parse().unwrap();

        moves.push(Move {
            count: first,
            from: third,
            to: fifth,
        });
    }

    return moves;
}

use std::collections::VecDeque;

fn move_crate_stack(all_stack: &mut Vec<VecDeque<&str>>, moves: Vec<Move>, is_part_2: bool) {
    for move_crate in moves {
        let from = (move_crate.from - 1) as usize;
        let to = (move_crate.to - 1) as usize;

        let mut crate_to_move: Vec<&str> = Vec::new();

        for _ in 0..move_crate.count {
            let mut ele = all_stack[from].pop_front().unwrap();
            if (!is_part_2) {
                all_stack[to].push_front(ele);
            } else {
                crate_to_move.push(ele);
            }
        }
        if (is_part_2) {
            crate_to_move.reverse();
            for cr in crate_to_move {
                all_stack[to].push_front(cr);
            }
        }
    }
}

fn main() {
    let is_part_2 = true;

    let content = read_input_file();
    let parsed = parse_stack(&content);
    let mut all_stack = make_stack_vectors(parsed.0, &parsed.1);

    let moves = parse_moves(&content);
    move_crate_stack(&mut all_stack, moves, is_part_2);

    print_stacks(&all_stack);
    // pas VPBQWRLBS, CVFVBLLSC
    for (pos, v) in all_stack.iter().enumerate().peekable() {
        let top_crate = v.front().unwrap();
        print!("{}", top_crate.chars().nth(1).unwrap());
    }

    // for (pos, v) in all_stack.iter().enumerate() {
    //     println!("stack: {}", pos + 1);

    //     for c in v {
    //         println!("{}", c);
    //     }
    // }
}

fn pop_from<'a>(all_vec: &mut Vec<Vec<&'a str>>, index: usize) -> &'a str {
    return all_vec[index].pop().unwrap();
}

fn print_stacks(all_vec: &Vec<VecDeque<&str>>) {
    for (pos, v) in all_vec.iter().enumerate() {
        println!("stack: {}", pos + 1);

        for c in v {
            println!("{}", c);
        }
    }
}
