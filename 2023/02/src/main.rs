//! Solution to Advent of Code 2023 - Day 2.
//!

use std::io;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

/// ```
/// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
///       |  \_____,_____/  \__________,_________/  \__,__/
///   index        hand                hand            hand
/// ```
#[derive(Debug, PartialEq)]
struct Game {
    index: u32,
    hands: Vec<Hand>,
}

impl Game {
    /// Returns true if all of the hands in the game are valid.
    fn is_valid(&self) -> bool {
        self.hands.iter().all(|h| h.is_valid())
    }

    /// Returns the product of the maximum numbers of red, green and blue cubes
    /// in all of the hands in the game.
    fn min_power(&self) -> u32 {
        self.hands.iter().map(|h| h.red).max().unwrap_or(0)
            * self.hands.iter().map(|h| h.green).max().unwrap_or(0)
            * self.hands.iter().map(|h| h.blue).max().unwrap_or(0)
    }

    fn parse(s: &str) -> Game {
        let (l, r) = s.split_once(": ").expect("Invalid game string");
        let index = l
            .get(5..)
            .expect("Invalid game string")
            .parse::<u32>()
            .expect("Invalid game ID");
        let hands = r
            .split(";")
            .map(|s| Hand::parse(s.trim()))
            .collect::<Vec<_>>();
        Game { index, hands }
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    /// Returns true if red, green and blue counts are all within the maximum thresholds.
    fn is_valid(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }

    /// Parses a Hand from its string representation.
    ///
    /// Examples:
    /// ```
    /// parse("1 red, 2 green, 3 blue"); // => Hand { red: 1, green: 2, blue: 3}
    /// parse("1 red"); // => Hand { red: 1, green: 0, blue: 0 }
    /// ```
    fn parse(s: &str) -> Hand {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for (a, b) in s
            .split(",")
            .map(|s| s.trim().split_once(" ").expect("Invalid colour"))
        {
            let i = a.parse::<u32>().expect("Invalid count");
            match b {
                "red" => red = i,
                "green" => green = i,
                "blue" => blue = i,
                _ => panic!("Invalid colour"),
            }
        }
        Hand { red, green, blue }
    }
}

fn main() {
    let games = io::stdin()
        .lines()
        .map(|line| Game::parse(&line.unwrap()))
        .collect::<Vec<_>>();

    let part_1_sum = games
        .iter()
        .filter(|g| g.is_valid())
        .map(|g| g.index)
        .sum::<u32>();
    println!("Part 1 sum: {}", part_1_sum);

    let part_2_sum = games.iter().map(|g| g.min_power()).sum::<u32>();
    println!("Part 2 sum: {}", part_2_sum);
}

#[test]
fn parse_hand() {
    assert_eq!(
        Hand::parse("1 red, 2 green, 3 blue"),
        Hand {
            red: 1,
            green: 2,
            blue: 3
        }
    );
}

#[test]
fn parse_partial_hand() {
    assert_eq!(
        Hand::parse("1 red"),
        Hand {
            red: 1,
            green: 0,
            blue: 0
        }
    );
}

#[test]
fn hand_is_valid() {
    assert!(Hand {
        red: MAX_RED,
        green: MAX_GREEN,
        blue: MAX_BLUE
    }
    .is_valid());
}

#[test]
fn hand_is_invalid() {
    assert!(!Hand {
        red: MAX_RED + 1,
        green: MAX_GREEN,
        blue: MAX_BLUE
    }
    .is_valid());
}

#[test]
fn parse_game() {
    assert_eq!(
        Game::parse("Game 42: 1 red; 2 green; 3 blue"),
        Game {
            index: 42,
            hands: vec! {
                Hand { red: 1, green: 0, blue: 0 },
                Hand { red: 0, green: 2, blue: 0 },
                Hand{ red: 0, green: 0, blue: 3 },
            },
        }
    );
}

#[test]
fn game_is_valid() {
    assert!(Game {
        index: 1,
        hands: vec! { Hand { red: 1, green: 2, blue: 3 } }
    }
    .is_valid())
}

#[test]
fn game_is_invalid() {
    assert!(!Game {
        index: 1,
        hands: vec! { Hand { red: MAX_RED + 1, green: 2, blue: 3 } }
    }
    .is_valid())
}

#[test]
fn game_min_power() {
    assert_eq!(
        48,
        Game {
            index: 1,
            hands: vec! {
                Hand { red: 4, green: 0, blue: 3 },
                Hand { red: 1, green: 2, blue: 6 },
                Hand { red: 0, green: 2, blue: 0 },
            }
        }
        .min_power()
    );
}
