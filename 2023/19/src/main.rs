use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static CHAIN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{([^}]+)\}").unwrap());
static PART_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap());

const ACCEPT: &str = "A";
const REJECT: &str = "R";

struct Chain {
    name: String,
    rules: Vec<Rule>,
}

impl Chain {
    fn parse(s: &str) -> Self {
        let caps = CHAIN_RE.captures(s).unwrap();
        let (name, rules_str) = (
            caps.get(1).unwrap().as_str().to_string(),
            caps.get(2).unwrap().as_str(),
        );
        let rules = rules_str.split(",").map(|s| Rule::parse(s)).collect();
        Chain { name, rules }
    }

    fn apply(&self, part: &Part) -> String {
        self.rules
            .iter()
            .map(|r| r.apply(part))
            .find(|r| r.is_some())
            .unwrap()
            .unwrap()
    }

    fn map(&self, range: &PartRange) -> Vec<(String, PartRange)> {
        let mut ret = vec![];
        let mut next = range.clone();
        for rule in self.rules.iter() {
            let (dest, matched, unmatched) = rule.map(&next);
            if let Some(r) = matched {
                ret.push((dest.clone(), r));
            }
            if let Some(r) = unmatched {
                next = r;
            } else {
                break;
            }
        }
        ret
    }
}

struct Rule {
    test: Option<Test>,
    destination: String,
}

struct Test {
    subject: Param,
    predicate: Pred,
}

impl Rule {
    fn parse(s: &str) -> Self {
        let (test, destination) = if s.contains(":") {
            let (test_str, dest) = s.split_once(":").unwrap();
            let subject = Param::from_char(test_str.chars().nth(0).unwrap());
            let predicate = Pred::parse(&test_str[1..]);
            (Some(Test { subject, predicate }), dest.to_string())
        } else {
            (None, s.to_string())
        };
        Rule { test, destination }
    }

    fn apply(&self, part: &Part) -> Option<String> {
        if let Some(Test { subject, predicate }) = &self.test {
            if predicate.matches(part.get(subject)) {
                Some(self.destination.clone())
            } else {
                None
            }
        } else {
            Some(self.destination.clone())
        }
    }

    fn map(&self, range: &PartRange) -> (String, Option<PartRange>, Option<PartRange>) {
        if let Some(Test { subject, predicate }) = &self.test {
            let (matched, non_matched) = predicate.map(range.get(subject));
            (
                self.destination.clone(),
                matched.map(|r| range.with(subject, r)),
                non_matched.map(|r| range.with(subject, r)),
            )
        } else {
            (self.destination.clone(), Some(range.clone()), None)
        }
    }
}

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn parse(s: &str) -> Self {
        let caps = PART_RE.captures(s).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let m = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let a = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let s = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        Part { x, m, a, s }
    }

    fn get(&self, p: &Param) -> i64 {
        match p {
            Param::X => self.x,
            Param::M => self.m,
            Param::A => self.a,
            Param::S => self.s,
        }
    }

    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn new() -> Self {
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn get(&self, p: &Param) -> (i64, i64) {
        match p {
            Param::X => self.x,
            Param::M => self.m,
            Param::A => self.a,
            Param::S => self.s,
        }
    }

    fn with(&self, p: &Param, r: (i64, i64)) -> Self {
        let mut other = self.clone();
        match p {
            Param::X => other.x = r,
            Param::M => other.m = r,
            Param::A => other.a = r,
            Param::S => other.s = r,
        }
        other
    }

    fn product(&self) -> i64 {
        (1 + self.x.1 - self.x.0)
            * (1 + self.m.1 - self.m.0)
            * (1 + self.a.1 - self.a.0)
            * (1 + self.s.1 - self.s.0)
    }
}

enum Param {
    X,
    M,
    A,
    S,
}

impl Param {
    fn from_char(c: char) -> Param {
        match c {
            'x' => Param::X,
            'm' => Param::M,
            'a' => Param::A,
            's' => Param::S,
            _ => unreachable!("Bad param"),
        }
    }
}

enum Pred {
    GreaterThan(i64),
    LessThan(i64),
}

impl Pred {
    fn matches(&self, val: i64) -> bool {
        match self {
            Pred::GreaterThan(t) => val > *t,
            Pred::LessThan(t) => val < *t,
        }
    }

    fn map(&self, (a, b): (i64, i64)) -> (Option<(i64, i64)>, Option<(i64, i64)>) {
        match self {
            Pred::GreaterThan(t) => {
                if *t >= a && *t < b {
                    (Some((t + 1, b)), Some((a, *t)))
                } else if *t >= b {
                    (None, Some((a, b)))
                } else {
                    // t < a
                    (Some((a, b)), None)
                }
            }
            Pred::LessThan(t) => {
                if *t > a && *t <= b {
                    (Some((a, *t - 1)), Some((*t, b)))
                } else if *t > b {
                    (Some((a, b)), None)
                } else {
                    // t <= a
                    (None, Some((a, b)))
                }
            }
        }
    }

    fn parse(s: &str) -> Self {
        let val = s[1..].parse::<i64>().unwrap();
        match s.chars().nth(0).unwrap() {
            '>' => Pred::GreaterThan(val),
            '<' => Pred::LessThan(val),
            _ => unreachable!("Bad predicate"),
        }
    }
}

fn part1(input: &str) -> i64 {
    let (chains_str, parts_str) = input.trim().split_once("\n\n").unwrap();
    let chains: HashMap<String, Chain> = chains_str
        .split("\n")
        .map(|s| Chain::parse(s))
        .map(|c| (c.name.clone(), c))
        .collect();
    let mut total = 0;
    for part in parts_str.split("\n").map(|s| Part::parse(s)) {
        let mut ch = "in".to_string();
        while ch != REJECT && ch != ACCEPT {
            ch = chains.get(&ch).unwrap().apply(&part);
        }
        if ch == ACCEPT {
            total += part.sum();
        }
    }
    total
}

fn part2(input: &str) -> i64 {
    let (chains_str, _) = input.trim().split_once("\n\n").unwrap();
    let chains: HashMap<String, Chain> = chains_str
        .split("\n")
        .map(|s| Chain::parse(s))
        .map(|c| (c.name.clone(), c))
        .collect();
    let mut total = 0;
    let mut ranges = vec![];
    ranges.push(("in".to_string(), PartRange::new()));
    while !ranges.is_empty() {
        ranges = ranges
            .iter()
            .map(|(ch, r)| chains.get(ch).unwrap().map(r))
            .flatten()
            .collect();
        total += ranges.iter().filter(|(ch, _)| ch == ACCEPT).map(|(_, r)| r.product()).sum::<i64>();
        ranges.retain(|(ch, _)| ch != ACCEPT && ch != REJECT);
    }
    total
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(19114, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(167409079868000, part2(TEST_INPUT));
    }
}
