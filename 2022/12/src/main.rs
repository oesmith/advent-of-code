use std::collections::VecDeque;
use std::env;
use std::io;

fn main() {
    let part2 = env::args().any(|x| x == "part2");

    let grid = Grid::from_stdin();

    let mut distances = vec![vec![-1; grid.width]; grid.height];
    let mut queue = VecDeque::new();

    let starts = if part2 { grid.part2_starts() } else { grid.part1_starts() };
    for (sx, sy) in starts {
        distances[sy][sx] = 0;
        queue.push_back((sx, sy));
    }

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let distance = distances[y][x];
        let elevation = grid.elevation(x, y);
        for (u, v) in grid.adjacent(x, y) {
            if distances[v][u] >= 0 {
                continue;  // Already visited.
            }
            if grid.elevation(u, v) <= elevation + 1 {
                if grid.get(u, v) == b'E' {
                    print!("Distance: {}\n", distance + 1);
                    return;
                }
                distances[v][u] = distance + 1;
                queue.push_back((u, v));
            }
        }
    }
}

struct Grid {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_stdin() -> Grid {
        let data: Vec<Vec<u8>> = io::stdin()
            .lines()
            .map(|l| l.unwrap().into_bytes())
            .collect();
        return Grid::create(data);
    }

    fn create(data: Vec<Vec<u8>>) -> Grid {
        let height = data.len();
        let width = data[0].len();
        return Grid {
            data,
            width,
            height,
        };
    }

    fn part1_starts(&self) -> Vec<(usize, usize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == b'S' {
                    return vec![(x, y)];
                }
            }
        }
        panic!();
    }

    fn part2_starts(&self) -> Vec<(usize, usize)> {
        let mut starts = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.elevation(x, y) == b'a' {
                    starts.push((x, y));
                }
            }
        }
        return starts;
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    fn elevation(&self, x: usize, y: usize) -> u8 {
        let v = self.get(x, y);
        return if v == b'S' { b'a' } else if v == b'E' { b'z' } else { v };
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        if x > 0 {
            ret.push((x - 1, y));
        }
        if x < self.width - 1 {
            ret.push((x + 1, y));
        }
        if y > 0 {
            ret.push((x, y - 1));
        }
        if y < self.height - 1 {
            ret.push((x, y + 1));
        }
        return ret;
    }
}
