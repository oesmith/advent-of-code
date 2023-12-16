use std::collections::VecDeque;

const EAST: u8 = 1;
const SOUTH: u8 = 2;
const WEST: u8 = 4;
const NORTH: u8 = 8;

struct Map {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(s: &str) -> Map {
        let tiles: Vec<Vec<char>> = s
            .trim()
            .split("\n")
            .into_iter()
            .map(|s| s.chars().collect())
            .collect();
        let (width, height) = (tiles[0].len(), tiles.len());
        Map {
            tiles,
            width,
            height,
        }
    }

    fn step(&self, row: usize, col: usize, dir: u8) -> Option<(usize, usize, u8)> {
        match dir {
            EAST => {
                if col < (self.width - 1) {
                    Some((row, col + 1, dir))
                } else {
                    None
                }
            },
            SOUTH => {
                if row < (self.height - 1) {
                    Some((row + 1, col, dir))
                } else {
                    None
                }
            },
            WEST => {
                if col > 0 {
                    Some((row, col - 1, dir))
                } else {
                    None
                }
            },
            NORTH => {
                if row > 0 {
                    Some((row - 1, col, dir))
                } else {
                    None
                }
            },
            _ => unreachable!("Bad direction"),
        }
    }

    fn next(&self, row: usize, col: usize, dir: u8) -> Vec<(usize, usize, u8)> {
        let dirs = match self.tiles[row][col] {
            '.' => vec![dir],
            '\\' => match dir {
                EAST => vec![SOUTH],
                SOUTH => vec![EAST],
                WEST => vec![NORTH],
                NORTH => vec![WEST],
                _ => unreachable!("Bad direction"),
            },
            '/' => match dir {
                EAST => vec![NORTH],
                SOUTH => vec![WEST],
                WEST => vec![SOUTH],
                NORTH => vec![EAST],
                _ => unreachable!("Bad direction"),
            },
            '-' => match dir {
                EAST | WEST => vec![dir],
                NORTH | SOUTH => vec![WEST, EAST],
                _ => unreachable!("Bad direction"),
            },
            '|' => match dir {
                NORTH | SOUTH => vec![dir],
                EAST | WEST => vec![NORTH, SOUTH],
                _ => unreachable!("Bad direction"),
            },
            _ => unreachable!("Bad tile"),
        };
        dirs.into_iter()
            .map(|d| self.step(row, col, d))
            .flatten()
            .collect()
    }

    fn beam(&self, r0: usize, c0: usize, d0: u8) -> usize {
        let mut visited: Vec<Vec<u8>> = vec![vec![0; self.width]; self.height];
        let mut queue: VecDeque<(usize, usize, u8)> = VecDeque::new();

        visited[r0][c0] = d0;
        queue.push_back((r0, c0, d0));
        while let Some((r, c, d)) = queue.pop_front() {
            for (r1, c1, d1) in self.next(r, c, d) {
                if visited[r1][c1] & d1 != 0 {
                    continue; // Don't get stuck in a loop.
                }
                visited[r1][c1] |= d1;
                queue.push_back((r1, c1, d1));
            }
        }

        visited.into_iter().flatten().filter(|n| *n > 0).count()
    }

    fn part1(&self) -> usize {
        self.beam(0, 0, EAST)
    }

    fn part2(&self) -> usize {
        let mut res: Vec<usize> = vec![];
        for i in 0..self.height {
            res.push(self.beam(i, 0, EAST));
            res.push(self.beam(i, self.width - 1, WEST));
        }
        for i in 0..self.width {
            res.push(self.beam(0, i, SOUTH));
            res.push(self.beam(self.width - 1, i, NORTH));
        }
        res.into_iter().max().unwrap()
    }
}

fn main() {
    let map = Map::parse(include_str!("../data/input.txt"));
    println!("Part 1: {}", map.part1());
    println!("Part 2: {}", map.part2());
}

#[cfg(test)]
mod test {
    use crate::Map;

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(46, Map::parse(TEST_INPUT).part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(51, Map::parse(TEST_INPUT).part2());
    }
}
