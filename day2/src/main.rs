// https://adventofcode.com/2022/day/2

/*
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/

/*
A, Rock, 1
B, Paper, 2
C, Scissors, 3

answer pas 16395

--- Part Two ---
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

*/
use std::{
    fs::File,
    io::{BufReader, Lines, Read},
};

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut fileContent = String::new();
    file.read_to_string(&mut fileContent);
    return fileContent;
}

#[derive(PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Draw,
    Lost,
    Win,
}

struct GameRound {
    pub myHand: Hand,
    pub elfHand: Hand,
}

impl Hand {
    fn hand_value(&self) -> i32 {
        return match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
    }

    fn get_win_hand(&self) -> Hand {
        return match self {
            Hand::Paper => Hand::Scissors,
            Hand::Rock => Hand::Paper,
            Hand::Scissors => Hand::Rock,
        };
    }

    fn get_lose_hand(&self) -> Hand {
        return match self {
            Hand::Paper => Hand::Rock,
            Hand::Rock => Hand::Scissors,
            Hand::Scissors => Hand::Paper,
        };
    }

    fn get_current_self_value(&self) -> Hand {
        return match self {
            Hand::Paper => Hand::Paper,
            Hand::Rock => Hand::Rock,
            Hand::Scissors => Hand::Scissors,
        };
    }

    fn get_hand_from_game_result(&self, wished_result: &str) -> Hand {
        return match wished_result {
            "X" => self.get_lose_hand(),
            "Y" => self.get_current_self_value(),
            "Z" => self.get_win_hand(),
            _ => panic!("sadsd"),
        };
    }

    fn get_score_from_hand(&self, other_hand: &Hand) -> i32 {
        if (self == other_hand) {
            return 3;
        }

        if (self == &Hand::Paper && other_hand == &Hand::Rock
            || self == &Hand::Rock && other_hand == &Hand::Scissors
            || self == &Hand::Scissors && other_hand == &Hand::Paper)
        {
            return 6;
        }

        return 0;
    }
}

impl GameRound {
    fn get_other_hand(input: &Vec<&str>) -> Hand {
        let elf_hand = match input[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("cannot parse hand"),
        };
        return elf_hand;
    }

    fn new(gameInput: &str, part2: bool) -> GameRound {
        let splitted: Vec<&str> = gameInput.split_whitespace().collect();
        let elf_hand = GameRound::get_other_hand(&splitted);

        let my_hand = if part2 {
            elf_hand.get_hand_from_game_result(splitted[1])
        } else {
            match splitted[1] {
                "X" => Hand::Rock,
                "Y" => Hand::Paper,
                "Z" => Hand::Scissors,
                _ => panic!("cannot parse hand"),
            }
        };
        // let my_hand = match splitted[1] {
        //     "X" => Hand::Rock,
        //     "Y" => Hand::Paper,
        //     "Z" => Hand::Scissors,
        //     _ => panic!("cannot parse hand"),
        // };

        return GameRound {
            myHand: my_hand,
            elfHand: elf_hand,
        };
    }

    pub fn get_game_result(&self) -> i32 {
        return self.myHand.get_score_from_hand(&self.elfHand) + self.myHand.hand_value();
    }
}

fn main() {
    let file_content = read_input_file();
    let mut all_game_round: Vec<GameRound> = Vec::new();
    let mut total = 0;
    for line in file_content.lines() {
        let game_round = GameRound::new(line, true);
        //all_game_round.push(game_round);
        total += game_round.get_game_result();
    }

    println!("result {} ", total);

    println!("Hello, world!");
}
