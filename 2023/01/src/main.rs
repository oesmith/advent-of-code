use std::io;

const LOOKUP: &[(&str, i64)] = &[
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_value(linestr: &str, n: usize) -> Option<i64> {
    let s = linestr.get(n..).unwrap();
    for (m, d) in LOOKUP {
        if s.starts_with(m) {
            return Some(*d);
        }
    }
    return None;
}

fn main() {
    let mut part_1_total: i64 = 0;
    let mut part_2_total: i64 = 0;

    let lines = io::stdin().lines();
    for line in lines {
        let _ = line.map(|linestr| {
            for i in 0..linestr.len() {
                let v = find_value(&linestr, i);
                if v.is_some() {
                    part_2_total += v.unwrap() * 10;
                    break;
                }
            }
            for i in 0..linestr.len() {
                let v = find_value(&linestr, linestr.len() - 1 - i);
                if v.is_some() {
                    part_2_total += v.unwrap();
                    break;
                }
            }
            linestr.chars()
                .find(|c| c.is_ascii_digit())
                .map(|c| part_1_total += i64::from(c.to_digit(10).unwrap()) * 10);
            linestr.chars()
                .rfind(|c| c.is_ascii_digit())
                .map(|c| part_1_total += i64::from(c.to_digit(10).unwrap()));
        });
    }

    println!("Part 1 answer: {}", part_1_total);
    println!("Part 2 answer: {}", part_2_total);
}
