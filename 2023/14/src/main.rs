use std::collections::HashMap;
use std::io::stdin;

const ITERATIONS: usize = 1_000_000_000;

fn cycle(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = input.clone();

    roll_north(&mut map);
    roll_west(&mut map);
    roll_south(&mut map);
    roll_east(&mut map);

    map
}

// TODO: find a sensible way to factor out the repetition in these four functions.

fn roll_north<'a>(map: &'a mut Vec<Vec<char>>) {
    let (height, width) = (map.len(), map[0].len());
    for c in 0..width {
        let mut last_space: Option<usize> = None;
        for r in 0..height {
            match map[r][c] {
                '.' => {
                    if last_space.is_none() {
                        last_space = Some(r);
                    }
                },
                '#' => last_space = None,
                'O' => {
                    if let Some(r0) = last_space {
                        (map[r0][c], map[r][c], last_space) = ('O', '.', Some(r0 + 1));
                    }
                },
                _ => unreachable!("Invalid tile")
            }
        }
    }
}

fn roll_west<'a>(map: &'a mut Vec<Vec<char>>) {
    let (height, width) = (map.len(), map[0].len());
    for r in 0..height {
        let mut last_space: Option<usize> = None;
        for c in 0..width {
            match map[r][c] {
                '.' => {
                    if last_space.is_none() {
                        last_space = Some(c);
                    }
                },
                '#' => last_space = None,
                'O' => {
                    if let Some(c0) = last_space {
                        (map[r][c0], map[r][c], last_space) = ('O', '.', Some(c0 + 1));
                    }
                },
                _ => unreachable!("Invalid tile")
            }
        }
    }
}

fn roll_south<'a>(map: &'a mut Vec<Vec<char>>) {
    let (height, width) = (map.len(), map[0].len());
    for c in 0..width {
        let mut last_space: Option<usize> = None;
        for r in (0..height).rev() {
            match map[r][c] {
                '.' => {
                    if last_space.is_none() {
                        last_space = Some(r);
                    }
                },
                '#' => last_space = None,
                'O' => {
                    if let Some(r0) = last_space {
                        (map[r0][c], map[r][c], last_space) = ('O', '.', Some(r0 - 1));
                    }
                },
                _ => unreachable!("Invalid tile")
            }
        }
    }
}

fn roll_east<'a>(map: &'a mut Vec<Vec<char>>) {
    let (height, width) = (map.len(), map[0].len());
    for r in 0..height {
        let mut last_space: Option<usize> = None;
        for c in (0..width).rev() {
            match map[r][c] {
                '.' => {
                    if last_space.is_none() {
                        last_space = Some(c);
                    }
                },
                '#' => last_space = None,
                'O' => {
                    if let Some(c0) = last_space {
                        (map[r][c0], map[r][c], last_space) = ('O', '.', Some(c0 - 1));
                    }
                },
                _ => unreachable!("Invalid tile")
            }
        }
    }
}

fn calculate_load(map: &Vec<Vec<char>>) -> usize {
    let (height, width) = (map.len(), map[0].len());
    let mut total = 0;
    for r in 0..height {
        for c in 0..width {
            if map[r][c] == 'O' {
                total += height - r;
            }
        }
    }
    total
}

fn main() {
    let map: Vec<Vec<char>> = stdin().lines().map(|r| r.unwrap().chars().collect()).collect();

    let mut part1_map = map.clone();
    roll_north(&mut part1_map);

    println!("Part 1 total: {}", calculate_load(&part1_map));

    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut cycle_discovered = false;
    let mut i = 0;
    let mut part2_map = map.clone();
    while i < ITERATIONS {
        if !cycle_discovered {
            if let Some(j) = cache.get(&part2_map) {
                i = ITERATIONS - ((ITERATIONS - i) % (i - j));
                cycle_discovered = true;
            }
        }
        cache.insert(part2_map.clone(), i);
        part2_map = cycle(part2_map);
        i += 1;
    }
    println!("Part 2 total: {}", calculate_load(&part2_map));
}
