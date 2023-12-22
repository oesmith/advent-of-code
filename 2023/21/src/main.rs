use std::collections::HashSet;

struct Map {
    walls: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
    positions: HashSet<(i64, i64)>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let map: Vec<Vec<char>> = input
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();
        let (height, width) = (map.len() as i64, map[0].len() as i64);
        let mut walls = HashSet::new();
        let mut positions = HashSet::new();

        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] == '#' {
                    walls.insert((row as i64, col as i64));
                }
                if map[row][col] == 'S' {
                    positions.insert((row as i64, col as i64));
                }
            }
        }

        Map {
            walls,
            width,
            height,
            positions,
        }
    }

    fn step(&mut self, iterations: i64) {
        for _ in 0..iterations {
            let mut new_positions = HashSet::new();
            for (r, c) in self.positions.iter() {
                let (wr, wc) = (r.rem_euclid(self.height), c.rem_euclid(self.width));
                if new_positions.contains(&(*r, *c))
                    || self.walls.contains(&(wr, wc)) {
                    continue;
                }
                for (dr, dc) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (r1, c1) = (r+dr, c+dc);
                    let (wr, wc) = (r1.rem_euclid(self.height), c1.rem_euclid(self.width));
                    if !new_positions.contains(&(r1, c1)) && !self.walls.contains(&(wr, wc)) {
                        new_positions.insert((r1, c1));
                    }
                }
            }
            self.positions = new_positions;
        }
    }

    fn num_reachable(&self) -> i64 {
        self.positions.len() as i64
    }
}

fn reachable(input: &str, iterations: i64) -> i64 {
    let mut map = Map::parse(input);
    map.step(iterations);
    map.num_reachable()
}

fn solve(input: &str, goal: i64) -> i64 {
    let mut map = Map::parse(input);
    assert_eq!(map.width, map.height);
    let dim = map.width;
    map.step(goal % dim);
    let a0 = map.num_reachable();
    map.step(dim);
    let a1 = map.num_reachable();
    map.step(dim);
    let a2 = map.num_reachable();
    let b0 = a1 - a0;
    let b1 = a2 - a1;
    let n = goal / dim;
    a0 + b0 * n + (n * (n - 1) / 2) * (b1 - b0)
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part 1: {}", reachable(input, 64));
    println!("Part 2: {}", solve(input, 26501365));
}

#[cfg(test)]
mod test {
    use crate::reachable;

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(16, reachable(TEST_INPUT, 6));
    }

    #[test]
    fn test_part2() {
        assert_eq!(16, reachable(TEST_INPUT, 6));
        assert_eq!(50, reachable(TEST_INPUT, 10));
        assert_eq!(1594, reachable(TEST_INPUT, 50));
    }
}
