#![allow(unused)]
/*
 * --- Day 1: Calorie Counting ---
Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
The second Elf is carrying one food item with 4000 Calories.
The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
The fifth Elf is carrying one food item with 10000 Calories.
In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
 *
 */


// use std::io::{self, Read};
// use std::fs::File;

// fn filename_to_string(s: &str) -> io::Result<String> {
//     let mut file = File::open(s)?;
//     let mut s = String::new();
//     file.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
//     s.lines().map(|line| {
//         line.split_whitespace().collect()
//     }).collect()
// }

// fn example_use() {
//     let whole_file = filename_to_string("terms.txt").unwrap();
//     let wbyl = words_by_line(&whole_file);
//     println!("{:?}", wbyl)
// }

use std::env;
use std::fs;
use std::io::{self, Read};
use std::fs::File;

fn file_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s);
    Ok(s)
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("correctly read input");

    let content2 = file_to_string("input.txt")
    .expect("Coulnd't read the input.txt");

    let elf_bags: Vec<&str> = content2.split("\n\n").collect();


    let mut max_so_far =0;
    for bag in elf_bags{
        let total_for_bag: i32 = bag.lines().map(|x| x.parse::<i32>().unwrap()).sum();
        max_so_far = if total_for_bag > max_so_far {total_for_bag} else {max_so_far};
    }

    println!("answer: {}", max_so_far)
    // let mut acc = 0;
    // for l in content2.unwrap().lines(){
    //     ifl.parse<i32>(){
    //         Ok(),
    //         Err()
    //     }
    //     println!("{}", l)
    // }


    // let split_on_2_newline = content.split("\n\n").collect();
    // for i in split_on_2_newline{
    //     print!("i1 {}",i)
    // }

    // let result = content.split('\n').map(|x| x.parse::<i32>().unwrap());

    // for i in result {
    //     println!("i {}", i)
    // }

    // println!("content: {content:?}");
}
