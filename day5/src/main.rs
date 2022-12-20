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
fn make_stack_vectors2<'a>(stack_values: Vec<&'a str>, total_stack: &i32) -> Vec<VecDeque<&'a str>> {
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
                stack.push_front(crate_string);
            }
        }
        all_stack.push(stack);
    }
    return all_stack;
}
// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicite lifetime
fn make_stack_vectors<'a>(stack_values: Vec<&'a str>, total_stack: &i32) -> Vec<Vec<&'a str>> {
    let mut all_stack: Vec<Vec<&'a str>> = Vec::new();

    // [R] [T] [T] [R] [G] [W] [F] [W] [L]
    // 0123456789
    //
    for stack_number in 0..*total_stack {
        let mut stack: Vec<&'a str> = Vec::new();
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
                stack.push(crate_string);
            }
        }
        stack.reverse();
        all_stack.push(stack);
    }
    return all_stack;
}

fn get_number_of_stack(stack_input: &str) -> i32 {
    let mut all_stack: Vec<Vec<&str>> = Vec::new();
    let mut stack_input_iterator = stack_input.lines().enumerate().peekable();

    while let Some((index, line)) = stack_input_iterator.next() {}

    let stack_input_lines: Vec<&str> = stack_input.split("\n").collect();
    let stack_number_line = stack_input_lines[stack_input_lines.len()];
    let total_stack = stack_number_line
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    return total_stack;
}

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

fn main() {
    let content = read_input_file();
    // let split: Vec<&str> = content.split("\n\n").collect();
    // let stack_input = split[0];
    // let stack_count = get_number_of_stack(stack_input);
    // let stack_input_lines: Vec<&str> = stack_input.split("\n").collect();
    let parsed = parse_stack(&content);
    let mut all_stack = make_stack_vectors2(parsed.0, &parsed.1);
    // for (pos, v) in all_stack.iter().enumerate(){
    //     println!("stack: {}", pos +1);

    //     for c in v{
    //         println!("{}", c);
    //     }
    // }

    let moves = parse_moves(&content);
    for move_crate in moves {
        let from = (move_crate.from - 1) as usize;
        {
            for _ in 0..move_crate.count {
                let mut ele = all_stack[from].pop_back().unwrap();
                all_stack[(move_crate.to - 1) as usize].push_front(ele);
            }
        }

    }

    for (pos, v) in all_stack.iter().enumerate() {
        println!("stack: {}", pos + 1);
        
        for c in v {
            println!("{}", c);
        }
    }
}

fn pop_from<'a>(all_vec: &mut Vec<Vec<&'a str>>, index: usize) -> &'a str {
    return all_vec[index].pop().unwrap();
}

fn print_stacks(all_vec: &Vec<Vec<&str>>){
   for (pos, v) in all_vec.iter().enumerate() {
        println!("stack: {}", pos + 1);

        for c in v {
            println!("{}", c);
        }
    } 
}