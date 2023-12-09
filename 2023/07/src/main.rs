use once_cell::sync::Lazy;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::io;

const PART1_ORDER: &str = "23456789TJQKA";
const PART1_LUT: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    PART1_ORDER
        .char_indices()
        .map(|(a, b)| (b, a as u8))
        .collect()
});

const PART2_ORDER: &str = "J23456789TQKA";
const PART2_LUT: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    PART2_ORDER
        .char_indices()
        .map(|(a, b)| (b, a as u8))
        .collect()
});

#[derive(Clone, Debug, PartialEq)]
struct Hand {
    cards: String,
    bet: u32,
}

impl Hand {
    fn parse(s: &str) -> Hand {
        let (cards, bet) = s.split_once(" ").unwrap();
        assert_eq!(5, cards.len());
        Hand {
            cards: cards.to_string(),
            bet: bet.parse().unwrap(),
        }
    }

    fn sort_key(&self, jokers: bool) -> (String, Vec<u8>) {
        (hand_type(&self.cards, jokers), card_scores(&self.cards, jokers))
    }
}

fn card_scores(cards: &String, jokers: bool) -> Vec<u8> {
    assert_eq!(5, cards.len());
    let lut = if jokers { PART2_LUT } else { PART1_LUT };
    cards.chars().map(|c| lut[&c]).collect()
}

fn hand_type(cards: &String, jokers: bool) -> String {
    assert_eq!(5, cards.len());
    let mut hist: HashMap<char, u32> = HashMap::new();
    let mut num_jokers = 0;
    for card in cards.chars() {
        if jokers && card == 'J' {
            num_jokers += 1;
        } else if let Some(x) = hist.get_mut(&card) {
            *x += 1;
        } else {
            hist.insert(card, 1);
        }
    }
    let mut vals = hist.into_values().collect::<Vec<_>>();
    vals.sort_by_key(|a| Reverse(*a));
    if jokers {
        if vals.is_empty() {
            vals.push(num_jokers);
        } else {
            *vals.get_mut(0).unwrap() += num_jokers;
        }
    }
    vals.into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn rank(hands: &mut Vec<Hand>, jokers: bool) {
    hands.sort_by_cached_key(|h| h.sort_key(jokers));
}

fn score(hands: &Vec<Hand>) -> u32 {
    let mut total = 0;
    for i in 0..hands.len() {
        total += (i as u32 + 1) * hands[i].bet;
    }
    total
}

fn main() {
    let mut hands: Vec<Hand> = io::stdin()
        .lines()
        .map(|l| Hand::parse(&l.unwrap()))
        .collect();

    rank(&mut hands, false);
    println!("Part 1 total: {}", score(&hands));

    rank(&mut hands, true);
    println!("Part 2 total: {}", score(&hands));
}

#[test]
fn test_hand_parse() {
    assert_eq!(
        Hand {
            cards: "23456".to_string(),
            bet: 123,
        },
        Hand::parse("23456 123")
    );
}

#[test]
fn test_card_scores() {
    assert_eq!(card_scores(&"23456".to_string(), false), vec![0, 1, 2, 3, 4]);
    assert_eq!(card_scores(&"TJQKA".to_string(), false), vec![8, 9, 10, 11, 12])
}

#[test]
fn test_card_scores_jokers() {
    assert_eq!(card_scores(&"23456".to_string(), true), vec![1, 2, 3, 4, 5]);
    assert_eq!(card_scores(&"TJQKA".to_string(), true), vec![9, 0, 10, 11, 12])
}

#[test]
fn test_hand_type() {
    assert_eq!("11111", hand_type(&"23456".to_string(), false));
    assert_eq!("311", hand_type(&"22256".to_string(), false));
    assert_eq!("221", hand_type(&"22344".to_string(), false));
    assert_eq!("32", hand_type(&"22244".to_string(), false));
    assert_eq!("2111", hand_type(&"22345".to_string(), false));
    assert_eq!("5", hand_type(&"AAAAA".to_string(), false));
}

#[test]
fn test_hand_type_jokers() {
    assert_eq!("11111", hand_type(&"23456".to_string(), true));
    assert_eq!("311", hand_type(&"22J56".to_string(), true));
    assert_eq!("221", hand_type(&"22344".to_string(), true));
    assert_eq!("32", hand_type(&"22J44".to_string(), true));
    assert_eq!("2111", hand_type(&"2J345".to_string(), true));
    assert_eq!("5", hand_type(&"AAAAJ".to_string(), true));
}

#[test]
fn test_hand_rank() {
    let a = Hand::parse("32T3K 765");
    let b = Hand::parse("T55J5 684");
    let c = Hand::parse("KK677 28");
    let d = Hand::parse("KTJJT 220");
    let e = Hand::parse("QQQJA 483");

    let mut ranked = vec![a.clone(), b.clone(), c.clone(), d.clone(), e.clone()];
    rank(&mut ranked, false);
    assert_eq!(vec![a, d, c, b, e], ranked);
}

#[test]
fn test_hand_rank_jokers() {
    let a = Hand::parse("32T3K 765");
    let b = Hand::parse("T55J5 684");
    let c = Hand::parse("KK677 28");
    let d = Hand::parse("KTJJT 220");
    let e = Hand::parse("QQQJA 483");

    let mut ranked = vec![a.clone(), b.clone(), c.clone(), d.clone(), e.clone()];
    rank(&mut ranked, true);
    assert_eq!(vec![a, c, b, e, d], ranked);
}
