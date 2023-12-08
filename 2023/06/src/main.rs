use regex::Regex;
use std::io;

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

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    assert!(lines.len() == 2);
    assert!(lines[0].starts_with("Time:"));
    assert!(lines[1].starts_with("Distance:"));

    let part1_times: Vec<u64> = re
        .find_iter(&lines[0])
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();
    let part1_distances: Vec<u64> = re
        .find_iter(&lines[1])
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    assert_eq!(part1_times.len(), part1_distances.len());

    println!("Part 1: {}", calculate(part1_times, part1_distances));

    let part2_time = lines[0]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let part2_distance = lines[1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    println!("Part 2: {}", calculate(vec![part2_time], vec![part2_distance]));
}
