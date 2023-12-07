use regex::Regex;
use std::io;

#[derive(Debug, PartialEq)]
struct RangeMap {
    a: u64,
    b: u64,
    len: u64,
}

impl RangeMap {
    fn intersects(&self, r: &SeedRange) -> bool {
        r.start < (self.a + self.len) && r.end() > self.a
    }

    fn forward(&self, r: &SeedRange) -> RangeMapResult {
        if self.intersects(r) {
            let left = if r.start < self.a {
                Some(SeedRange {
                    start: r.start,
                    len: self.a - r.start,
                })
            } else {
                None
            };
            let right = if r.end() > (self.a + self.len) {
                Some(SeedRange {
                    start: self.a + self.len,
                    len: r.end() - self.a - self.len,
                })
            } else {
                None
            };
            let mapped = Some(SeedRange {
                start: self.b + self.a.max(r.start) - self.a,
                len: r.end().min(self.a + self.len) - self.a.max(r.start),
            });
            RangeMapResult {
                mapped,
                left,
                right,
            }
        } else {
            RangeMapResult {
                mapped: None,
                left: Some(r.clone()),
                right: None,
            }
        }
    }

    fn parse(s: &str) -> RangeMap {
        let fields = s
            .split(" ")
            .map(|f| f.parse::<u64>().expect("Invalid number"))
            .collect::<Vec<_>>();
        debug_assert_eq!(3, fields.len());
        RangeMap {
            a: fields[1],
            b: fields[0],
            len: fields[2],
        }
    }
}

#[derive(Debug, PartialEq)]
struct RangeMapResult {
    mapped: Option<SeedRange>,
    left: Option<SeedRange>,
    right: Option<SeedRange>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SeedRange {
    start: u64,
    len: u64,
}

impl SeedRange {
    fn parse_ranges(s: &str) -> Vec<SeedRange> {
        debug_assert!(s.starts_with("seeds: "));
        let vals = s[7..]
            .split(" ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut out = vec![];
        for i in 0..(vals.len() / 2) {
            out.push(SeedRange {
                start: vals[i * 2],
                len: vals[i * 2 + 1],
            });
        }
        out
    }

    fn end(&self) -> u64 {
        self.start + self.len
    }
}

fn main() {
    let seeds_re: Regex = Regex::new("^seeds:( \\d+)+$").unwrap();
    let layer_re: Regex = Regex::new("^\\w+-to-\\w+ map:$").unwrap();
    let range_re: Regex = Regex::new("^\\d+ \\d+ \\d+$").unwrap();

    let mut prev: Vec<SeedRange> = vec![];
    let mut next: Vec<SeedRange> = vec![];

    for line in io::stdin().lines().map(|l| l.unwrap()) {
        match line {
            l if l.is_empty() => continue,
            l if seeds_re.is_match(&l) => {
                next.extend(SeedRange::parse_ranges(&l));
            }
            l if layer_re.is_match(&l) => {
                prev.append(&mut next);
            }
            l if range_re.is_match(&l) => {
                let rm = RangeMap::parse(&l);
                let mut unmapped = vec![];
                for r in prev.drain(..) {
                    if rm.intersects(&r) {
                        let res = rm.forward(&r);
                        if let Some(mapped) = res.mapped {
                            next.push(mapped);
                        }
                        if let Some(left) = res.left {
                            unmapped.push(left);
                        }
                        if let Some(right) = res.right {
                            unmapped.push(right);
                        }
                    } else {
                        unmapped.push(r);
                    }
                }
                prev.append(&mut unmapped);
            }
            _ => unreachable!("Invalid line"),
        }
    }

    prev.append(&mut next);

    println!("Min: {}", prev.iter().map(|r| r.start).min().unwrap());
}

#[test]
fn test_range_map_parse() {
    assert_eq!(
        RangeMap {
            a: 20,
            b: 10,
            len: 5
        },
        RangeMap::parse("10 20 5")
    );
}

#[test]
fn test_range_map_intersects() {
    assert!(RangeMap {
        a: 10,
        b: 20,
        len: 5
    }
    .intersects(&SeedRange { start: 11, len: 2 }));
    assert!(!RangeMap {
        a: 10,
        b: 20,
        len: 5
    }
    .intersects(&SeedRange { start: 5, len: 5 }));
}

#[test]
fn test_range_map_forwards() {
    // Non-intersecting.
    assert_eq!(
        RangeMapResult {
            mapped: None,
            left: Some(SeedRange { start: 5, len: 5 }),
            right: None,
        },
        RangeMap {
            a: 10,
            b: 50,
            len: 20,
        }
        .forward(&SeedRange { start: 5, len: 5 })
    );
    // Overlapping left.
    assert_eq!(
        RangeMapResult {
            mapped: Some(SeedRange { start: 50, len: 5 }),
            left: Some(SeedRange { start: 5, len: 5 }),
            right: None,
        },
        RangeMap {
            a: 10,
            b: 50,
            len: 20,
        }
        .forward(&SeedRange { start: 5, len: 10 })
    );
    // Map is a sub-set of input range.
    assert_eq!(
        RangeMapResult {
            mapped: Some(SeedRange { start: 50, len: 20 }),
            left: Some(SeedRange { start: 5, len: 5 }),
            right: Some(SeedRange { start: 30, len: 5 }),
        },
        RangeMap {
            a: 10,
            b: 50,
            len: 20,
        }
        .forward(&SeedRange { start: 5, len: 30 })
    );
    // Exactly equal.
    assert_eq!(
        RangeMapResult {
            mapped: Some(SeedRange { start: 50, len: 20 }),
            left: None,
            right: None,
        },
        RangeMap {
            a: 10,
            b: 50,
            len: 20,
        }
        .forward(&SeedRange { start: 10, len: 20 }));
    // Range is a sub-set of map.
    assert_eq!(
        RangeMapResult {
            mapped: Some(SeedRange { start: 12, len: 8 }),
            left: None,
            right: None,
        },
        RangeMap {
            a: 50,
            b: 10,
            len: 20,
        }
        .forward(&SeedRange { start: 52, len: 8 }));
}

#[test]
fn test_parse_range() {
    assert_eq!(
        vec![
            SeedRange { start: 10, len: 20 },
            SeedRange { start: 40, len: 5 }
        ],
        SeedRange::parse_ranges("seeds: 10 20 40 5")
    );
}
