/*
--- Day 3: Rucksack Reorganization ---
One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
The fourth rucksack's compartments only share item type v.
The fifth rucksack's compartments only share item type t.
The sixth rucksack's compartments only share item type s.
To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
*/
use std::{fs::File, io::Read};

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut fileContent = String::new();
    file.read_to_string(&mut fileContent);
    return fileContent;
}

static ALPHABET_VEC: [&'static char; 52] = [
    &'a', &'b', &'c', &'d', &'e', &'f', &'g', &'h', &'i', &'j', &'k', &'l', &'m', &'n', &'o', &'p',
    &'q', &'r', &'s', &'t', &'u', &'v', &'w', &'x', &'y', &'z', &'A', &'B', &'C', &'D', &'E', &'F',
    &'G', &'H', &'I', &'J', &'K', &'L', &'M', &'N', &'O', &'P', &'Q', &'R', &'S', &'T', &'U', &'V',
    &'W', &'X', &'Y', &'Z',
];

fn get_priority(&letter: &char) -> i32 {
    let index = ALPHABET_VEC.iter().position(|&r| r == &letter).unwrap();
    return index as i32 + 1;
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_rucksack_compartment(line: &str) -> (&str, &str) {
    let splitted = line.split_at(line.len() / 2);
    return splitted;
}

fn get_share_values(compartments: (&str, &str)) -> Option<char> {
    let left = compartments.0;
    let right = compartments.1;
    let mut shared: Vec<char> = Vec::new();

    for c in left.chars() {
        if (right.contains(c)) {
            return Some(c);
            //shared.push(c);
        }
    }

    return None;

    //    return shared;
}

fn main() {
    let input = read_input_file();
    let lines = input.lines();
    let mut all_shared: Vec<char> = Vec::new();

    for line in lines {
        let compartments = get_rucksack_compartment(&line);
        let shared = get_share_values(compartments);
        all_shared.push(shared.unwrap());
    }

    let result: i32 = all_shared.iter().map(|x| get_priority(x)).sum();
    println!("{}", result);
}
