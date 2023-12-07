use regex::Regex;
use std::io;

#[derive(Debug, PartialEq)]
struct RangeMap {
    a: u64,
    b: u64,
    len: u64,
}

impl RangeMap {
    fn contains(&self, i: u64) -> bool {
        i >= self.a && i < self.a + self.len
    }

    fn forward(&self, i: u64) -> Option<u64> {
        if self.contains(i) {
            Some(self.b + i - self.a)
        } else {
            None
        }
    }

    fn parse(s: &str) -> RangeMap {
        let fields = s
            .split(" ")
            .map(|f| f.parse::<u64>().expect("Invalid number"))
            .collect::<Vec<_>>();
        debug_assert_eq!(3, fields.len());
        RangeMap { a: fields[1], b: fields[0], len: fields[2] }
    }
}

fn parse_seeds(s: &str) -> Vec<u64> {
    debug_assert!(s.starts_with("seeds: "));
    s[7..].split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

fn main() {
    let seeds_re: Regex = Regex::new("^seeds:( \\d+)+$").unwrap();
    let layer_re: Regex = Regex::new("^\\w+-to-\\w+ map:$").unwrap();
    let range_re: Regex = Regex::new("^\\d+ \\d+ \\d+$").unwrap();

    let mut prev: Vec<u64> = vec![];
    let mut next: Vec<u64> = vec![];

    for line in io::stdin().lines().map(|l| l.unwrap()) {
        match line {
            l if l.is_empty() => continue,
            l if seeds_re.is_match(&l) => {
                next = parse_seeds(&l);
            },
            l if layer_re.is_match(&l) => {
                prev.append(&mut next);
            },
            l if range_re.is_match(&l) => {
                let rm = RangeMap::parse(&l);
                next.extend(prev.iter().filter_map(|p| rm.forward(*p)));
                prev.retain(|p| !rm.contains(*p));
            },
            _ => unreachable!("Invalid line"),
        }
    }

    prev.append(&mut next);

    println!("Min: {}", prev.iter().min().unwrap());
}

#[test]
fn test_range_map_parse() {
    assert_eq!(RangeMap { a: 20, b: 10, len: 5}, RangeMap::parse("10 20 5"));
}

#[test]
fn test_range_map_contains() {
    assert!(RangeMap { a: 10, b: 20, len: 5 }.contains(12));
    assert!(!RangeMap { a: 10, b: 20, len: 5 }.contains(15));
}

#[test]
fn test_range_map_forwards() {
    assert_eq!(Some(22), RangeMap { a: 10, b: 20, len: 5 }.forward(12));
    assert_eq!(None, RangeMap { a: 10, b: 20, len: 5 }.forward(15));
}

#[test]
fn test_parse_seeds() {
    assert_eq!(vec![10, 20, 30], parse_seeds("seeds: 10 20 30"));
}
