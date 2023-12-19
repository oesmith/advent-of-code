use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Eq, PartialEq)]
struct ByRow((i64, i64, i64));

impl Ord for ByRow {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0 .0.cmp(&self.0 .0)
    }
}

impl PartialOrd for ByRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct ByCol((i64, i64, i64));

impl Ord for ByCol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0 .1.cmp(&other.0 .1)
    }
}

impl PartialOrd for ByCol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fill(input: Vec<(char, i64)>) -> i64 {
    // A list of vertical lines in the grid - (row0, col0, len).
    let mut edges = BinaryHeap::new();
    let (mut row, mut col) = (0, 0);
    let (mut rowmin, mut rowmax) = (0, 0);

    for (dir, len) in input {
        match dir {
            'D' => {
                edges.push(ByRow((row, col, len)));
                row += len;
            }
            'U' => {
                row -= len;
                edges.push(ByRow((row, col, len)));
            }
            'L' => {
                col -= len;
            }
            'R' => {
                col += len;
            }
            _ => unreachable!("Invalid direction"),
        }
        rowmin = rowmin.min(row);
        rowmax = rowmax.max(row);
    }

    let mut active = BTreeSet::new();
    let mut count = 0;
    // This can probably go faster by not iterating over _every_ row, but it works within
    // reasonable time on my machine, so it's good enough for today.
    for row in rowmin..=rowmax {
        while let Some(ByRow((r, _, _))) = edges.peek() {
            if *r == row {
                active.insert(ByCol(edges.pop().unwrap().0));
            } else {
                break;
            }
        }
        active.retain(|ByCol((r, _, l))| r + l >= row);

        let mut inside = false;
        let mut col0 = 0;
        let mut prev_corner = None;
        let mut prev_inside = false;
        for ByCol((r, c, l)) in active.iter() {
            if !inside {
                col0 = *c;
                prev_inside = inside;
                inside = true;
                if row == *r || row == r + l {
                    prev_corner = Some((r, c, l));
                }
            } else if let Some((r0, _, l0)) = prev_corner {
                prev_corner = None;
                if row == *r0 && row == *r || row == (r0 + l0) && row == (r + l) {
                    inside = prev_inside;
                    if inside == false {
                        count += 1 + c - col0;
                    }
                } else {
                    inside = !prev_inside;
                    if inside == false {
                        count += 1 + c - col0;
                    }
                }
            } else if row == *r || row == r + l {
                prev_inside = inside;
                prev_corner = Some((r, c, l));
            } else {
                inside = false;
                count += 1 + c - col0;
            }
        }
    }

    count
}

fn part1(input: &str) -> i64 {
    fill(input.trim().split("\n").map(|s| {
        let [dir, ls, _] = s.split(" ").collect::<Vec<_>>()[..] else {
            unreachable!("Bad input")
        };
        (dir.chars().nth(0).unwrap(), ls.parse::<i64>().unwrap())
    }).collect::<Vec<_>>())
}

fn part2(input: &str) -> i64 {
    fill(input.trim().split("\n").map(|s| {
        let [_, _, c] = s.split(" ").collect::<Vec<_>>()[..] else {
            unreachable!("Bad input")
        };
        assert_eq!(9, c.len());
        let d = match c.chars().nth(7).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!("Bad direction")
        };
        let l = i64::from_str_radix(&c[2..7], 16).unwrap();
        (d, l)
    }).collect::<Vec<_>>())
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
        assert_eq!(62, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(952408144115, part2(TEST_INPUT));
    }
}
