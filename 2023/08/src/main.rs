use regex::Regex;
use std::collections::HashMap;
use std::io;

fn parse_input() -> (String, HashMap<String, (String, String)>) {
    let directions_re: Regex = Regex::new("^[LR]+$").unwrap();
    let nodes_re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();

    let mut directions: String = String::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in io::stdin().lines() {
        let l = line.unwrap();
        if directions_re.is_match(&l) {
            directions.push_str(&l);
        }
        if let Some(g) = nodes_re.captures(&l) {
            let name = g.get(1).unwrap().as_str().to_string();
            let left = g.get(2).unwrap().as_str().to_string();
            let right = g.get(3).unwrap().as_str().to_string();
            nodes.insert(name, (left, right));
        }
    }

    (directions, nodes)
}

fn step_count(
    directions: &String,
    nodes: &HashMap<String, (String, String)>,
    start: &String,
    end: fn(&String) -> bool,
) -> u64 {
    let mut position: String = start.clone();
    let mut counter: u64 = 0;
    loop {
        let direction = directions
            .chars()
            .nth((counter % directions.len() as u64) as usize)
            .unwrap();
        let (left, right) = nodes.get(&position).unwrap();
        position = match direction {
            'L' => left.clone(),
            'R' => right.clone(),
            _ => unreachable!("bad direction"),
        };
        counter += 1;
        if end(&position) {
            return counter;
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let (mut gcd, mut tmp) = (a, b);
    while tmp != 0 {
        (gcd, tmp) = (tmp, gcd % tmp);
    }
    return (a / gcd) * b;
}

fn main() {
    let (directions, nodes) = parse_input();

    if nodes.contains_key(&"AAA".to_string()) && nodes.contains_key(&"ZZZ".to_string()) {
        println!(
            "Part 1 count: {}",
            step_count(&directions, &nodes, &"AAA".to_string(), |p| *p == "ZZZ".to_string())
        );
    }

    let count = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| step_count(&directions, &nodes, k, |p| p.ends_with("Z")))
        .reduce(|a, c| lcm(a, c))
        .unwrap();
    println!("Part 2 count: {}", count);
}
