use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let mut orig_edges = vec![];
    let mut orig_nodes = HashSet::new();

    for (from, to_list) in input
        .trim()
        .split("\n")
        .map(|s| s.split_once(": ").unwrap())
    {
        orig_nodes.insert(from);
        for to in to_list.split(" ") {
            orig_nodes.insert(to);
            orig_edges.push((from, to));
        }
    }

    let mut rng = thread_rng();

    // Rough implementation of
    // https://en.wikipedia.org/wiki/Karger%27s_algorithm
    loop {
        let mut edges = orig_edges.clone();
        let mut nodes = orig_nodes.clone();
        let mut counts = nodes.iter().map(|n| (*n, 1)).collect::<HashMap<_, _>>();
        while nodes.len() > 2 {
            let idx = rng.gen_range(0..edges.len());
            let (from, to) = edges.remove(idx);
            for i in 0..edges.len() {
                if edges[i].0 == to {
                    edges[i].0 = from;
                }
                if edges[i].1 == to {
                    edges[i].1 = from;
                }
            }
            edges.retain_mut(|(a, b)| a != b);
            nodes.remove(&to);
            counts.insert(from, counts.get(from).unwrap() + counts.get(to).unwrap());
        }
        if edges.len() == 3 {
            return nodes.iter().map(|n| counts.get(n).unwrap()).product();
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part 1: {}", solve(input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_solve() {
        assert_eq!(54, solve(TEST_INPUT));
    }
}
