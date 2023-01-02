/*
--- Day 8: Treetop Tree House ---
The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390
Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
The top-middle 5 is visible from the top and right.
The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
The left-middle 5 is visible, but only from the right.
The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
The right-middle 3 is visible from the right.
In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?



*/

use std::{fs::File, io::Read};

fn read_file() -> String {
    let mut file = File::open("input-test.txt").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content);
    return file_content;
}

fn parse_content(content: String) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in content.lines() {
        let line_vector: Vec<i32> = line
            .chars()
            .enumerate()
            .map(|c| c.1 as i32 - 0x30)
            .collect();
        grid.push(line_vector);
    }

    return grid;
}

enum DIR {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
}

impl DIR {
    fn get_step(&self) -> (i8, i8) {
        return match &self {
            DIR::LEFT => (0, -1),
            DIR::RIGHT => (0, 1),
            DIR::TOP => (-1, 0),
            DIR::BOTTOM => (1, 0),
            _ => panic!("woups"),
        };
    }

    fn apply_step(&self, x: usize, y: usize) -> (usize, usize) {
        let step_diff = self.get_step();
        let size_x_to_add: usize = step_diff.0 as usize;
        return ((x + size_x_to_add) as usize, y + (step_diff.1 as usize));
    }
}

fn is_ordered_increasing(v: &Vec<i32>) -> bool {
    v.windows(2).all(|w| w[0] < w[1])
}

fn is_ordered_decreasing(v: &Vec<i32>) -> bool {
    v.windows(2).all(|w| w[0] > w[1])
}

fn is_visible_direction(x: usize, y: usize, direction: &DIR, grid: &Vec<Vec<i32>>) -> bool {
    let step = direction.get_step();
    let mut x1 = x;
    let mut y1 = y;

    let visible = match direction  {
        DIR::TOP => {
            let top: Vec<i32> = (&grid[0..x + 1]).iter().map(|i| i[y]).collect();
            return is_ordered_increasing(&top);
        },
        DIR::BOTTOM => {
            let bottom: Vec<i32> = (&grid[x..]).iter().map(|i| i[y]).collect();
            return is_ordered_decreasing(&bottom);
        },
        DIR::LEFT => {
            let left :Vec<i32> = grid[x][0..y+1].to_vec();
            return is_ordered_increasing(&left);
        },
        DIR::RIGHT => {
            let right = grid[x][y..].to_vec();
            return is_ordered_decreasing(&right);
        },
        _ => unreachable!(),
    };

    return visible;


    // // let top = (&grid[0..x + 1]).iter().map(|i| i[y]).collect();
    // // println!("top");
    // // print_vec(&top);

    // let bottom = (&grid[x..]).iter().map(|i| i[y]).collect();
    // println!("bottom");
    // print_vec(&bottom);
    
    // let left :Vec<i32> = grid[x][0..y+1].to_vec();
    // println!("left");
    // print_vec(&left);

    // let right = grid[x][y..].to_vec();
    // println!("right");
    // print_vec(&right);


    // let mut last_value = grid[x][y];
    // let mut curent_step = direction.apply_step(x, y);

    // while let Some(item) = grid.get(curent_step.0).and_then(|x| x.get(curent_step.1)) {
    //     if (*item >= last_value) {
    //         return false;
    //     }
    // }

    // return true;
}

fn is_visible(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> bool {
    let mut visible = false;

    let mut x1 = x;
    let mut y2 = y;

    visible = is_visible_direction(x, y, &DIR::BOTTOM, grid)
        || is_visible_direction(x, y, &DIR::LEFT, grid)
        || is_visible_direction(x, y, &DIR::RIGHT, grid)
        || is_visible_direction(x, y, &DIR::TOP, grid);

    return visible;
}

/*

30373
25512
65332
33549
35390
*/
fn main() {
    println!("Hello, world!");

    let content = read_file();
    let grid = parse_content(content);

    let x = 2;
    let y = 3;
    let val = grid[x][y];

    let top = (&grid[0..x + 1]).iter().map(|i| i[y]).collect();
    println!("top");
    print_vec(&top);

    let bottom = (&grid[x..]).iter().map(|i| i[y]).collect();
    println!("bottom");
    print_vec(&bottom);
    
    let left :Vec<i32> = grid[x][0..y+1].to_vec();
    println!("left");
    print_vec(&left);

    let right = grid[x][y..].to_vec();
    println!("right");
    print_vec(&right);

    let is_visible = is_visible_direction(x, y, &DIR::RIGHT, &grid);
    println!("visible? {} ", is_visible );

    //let is_visible = is_visible(x,y, &grid);

    //    println!("[{},{}]is_visible: {}", x, y, is_visible );
}

fn print_vec(v: &Vec<i32>) {
    let s: String = v
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", s);
}
