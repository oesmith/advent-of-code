use std::collections::HashSet;
use std::io;

fn parse_set(s: &str) -> HashSet<usize> {
    s.trim()
        .split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<usize>().expect("Invalid score"))
        .collect()
}

fn match_count(line: &str) -> usize {
    let (l, r) = line
        .split_once(":")
        .expect("Invalid game string")
        .1
        .split_once("|")
        .expect("Invalid game string");
    let ls = parse_set(l);
    let rs = parse_set(r);
    ls.intersection(&rs).count()
}

fn part_1(scores: &Vec<usize>) -> usize {
    scores.iter().filter(|s| **s > 0).map(|s| 2_usize.pow((*s as u32)-1)).sum::<usize>()
}

fn part_2(scores: &Vec<usize>) -> usize {
    let mut card_counts = vec![1; scores.len()];

    for i in 0..card_counts.len() {
        let i1 = i + 1;
        let i2 = (i1 + scores[i]).min(scores.len());
        for j in i1..i2 {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.iter().sum::<usize>()
}

fn main() {
    let scores = io::stdin()
        .lines()
        .map(|l| match_count(&l.unwrap()))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&scores));
    println!("Part 1: {}", part_2(&scores));
}

#[test]
fn test_parse_set() {
    assert_eq!(
        HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        parse_set("83 86  6 31 17  9 48 53")
    );
}

#[test]
fn test_match_count() {
    assert_eq!(
        4,
        match_count("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
    );

    assert_eq!(
        0,
        match_count("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
    );
}
