use std::collections::{HashMap, HashSet};
use std::env;
use std::io;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Vec(i32, i32);

fn main() {
    let dirs = HashMap::from([
        ("U", Vec(0, 1)),
        ("D", Vec(0, -1)),
        ("L", Vec(-1, 0)),
        ("R", Vec(1, 0)),
    ]);

    let length = if env::args().any(|x| x == "ten") { 10 } else { 2 };
    let last = length - 1;

    let mut rope = vec![Vec(0, 0); length];
    let mut visited = HashSet::from([Vec(0, 0)]);
    let mut unique_visited_count = 1;

    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();
        let (dir_str, count_str) = line_str.split_once(' ').unwrap();
        let count = count_str.parse().unwrap();
        let dir = dirs.get(dir_str).unwrap();
        for _ in 0..count {
            rope[0] = Vec(rope[0].0 + dir.0, rope[0].1 + dir.1);
            for i in 0..last {
                let a = rope[i];
                let b = rope[i+1];
                let diff = Vec(a.0 - b.0, a.1 - b.1);
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    rope[i+1] = Vec(b.0 + step(diff.0), b.1 + step(diff.1));
                }
            }
            if !visited.contains(&rope[last]) {
                unique_visited_count += 1;
                visited.insert(rope[last]);
            }
        }
    }
    print!("Visited {}\n", unique_visited_count);
}

fn step(i: i32) -> i32 {
    return if i == 0 { 0 } else { i / i.abs() };
}
