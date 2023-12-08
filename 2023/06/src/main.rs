use once_cell::sync::Lazy;
use regex::Regex;
use std::io;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

fn calculate(times: Vec<u64>, distances: Vec<u64>) -> u64 {
    times
        .iter()
        .zip(distances)
        .map(|(t, d)| {
            let n = (0..*t).into_iter().filter(|i| i * (t - i) > d).count();
            println!("t:{} d:{} > {}", t, d, n);
            n
        })
        .reduce(|a, n| a * n)
        .unwrap() as u64
}

fn parse_part1(l: &str) -> Vec<u64> {
    RE.find_iter(l)
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect()
}

fn parse_part2(l: &str) -> Vec<u64> {
    vec![l.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap()]
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    assert!(lines.len() == 2);
    assert!(lines[0].starts_with("Time:"));
    assert!(lines[1].starts_with("Distance:"));

    let part1_times = parse_part1(&lines[0]);
    let part1_distances = parse_part1(&lines[1]);

    assert_eq!(part1_times.len(), part1_distances.len());

    println!("Part 1: {}", calculate(part1_times, part1_distances));

    let part2_times = parse_part2(&lines[0]);
    let part2_distances = parse_part2(&lines[1]);

    println!("Part 2: {}", calculate(part2_times, part2_distances));
}
