use std::collections::{HashMap, VecDeque};

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
            .map(|row| row.chars().collect())
            .collect();
        let (height, width) = (tiles.len(), tiles[0].len());
        Map {
            tiles,
            height,
            width,
        }
    }

    fn start_pos(&self) -> (usize, usize) {
        for c in 0..self.width {
            if self.tiles[0][c] == '.' {
                return (0, c);
            }
        }
        unreachable!("No start point");
    }

    fn end_pos(&self) -> (usize, usize) {
        for c in 0..self.width {
            if self.tiles[self.height - 1][c] == '.' {
                return (self.height - 1, c);
            }
        }
        unreachable!("No end point");
    }

    fn neighbours(&self, (r, c): (usize, usize), dir: bool) -> Vec<(usize, usize)> {
        let mut next = vec![];
        if r > 0
            && (!dir && self.tiles[r - 1][c] != '#'
                || self.tiles[r - 1][c] == '.'
                || self.tiles[r - 1][c] == '^')
        {
            next.push((r - 1, c));
        }
        if r + 1 < self.height
            && (!dir && self.tiles[r + 1][c] != '#'
                || self.tiles[r + 1][c] == '.'
                || self.tiles[r + 1][c] == 'v')
        {
            next.push((r + 1, c));
        }
        if c > 0
            && (!dir && self.tiles[r][c - 1] != '#'
                || self.tiles[r][c - 1] == '.'
                || self.tiles[r][c - 1] == '<')
        {
            next.push((r, c - 1));
        }
        if c + 1 < self.width
            && (!dir && self.tiles[r][c + 1] != '#'
                || self.tiles[r][c + 1] == '.'
                || self.tiles[r][c + 1] == '>')
        {
            next.push((r, c + 1));
        }
        next
    }

    fn max_path_len(&self, dir: bool) -> usize {
        let start = self.start_pos();
        let end = self.end_pos();

        let mut nodes: HashMap<(usize, usize), usize> = HashMap::new();
        nodes.insert(start, 0);
        nodes.insert(end, 1);
        for r in 0..self.height {
            for c in 0..self.width {
                if self.tiles[r][c] != '#' && self.neighbours((r, c), false).len() > 2 {
                    nodes.insert((r, c), nodes.len());
                }
            }
        }

        let mut edges: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
        for (p, n) in nodes.iter() {
            let mut node_edges = HashMap::new();
            let mut queue = VecDeque::new();
            for p1 in self.neighbours(*p, dir) {
                queue.push_back((*p, p1, 1));
            }
            while let Some((prev, cur, len)) = queue.pop_front() {
                if let Some(dest) = nodes.get(&cur) {
                    node_edges.insert(*dest, len);
                } else {
                    let neighbours = self.neighbours(cur, dir);
                    for next in neighbours.iter().filter(|nb| **nb != prev) {
                        queue.push_back((cur, *next, len + 1));
                    }
                }
            }
            edges.insert(*n, node_edges);
        }

        let mut complete = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, 1 as u64, 0));
        while let Some((node, visited, len)) = queue.pop_front() {
            if node == 1 {
                complete.push(len);
                continue;
            }
            let neighbours = edges.get(&node).unwrap();
            for edge in neighbours.iter().filter(|p| visited & (1 << p.0) == 0) {
                queue.push_back((*edge.0, visited + (1 << edge.0), len + edge.1));
            }
        }

        *complete.iter().max().unwrap()
    }
}

fn main() {
    let map = Map::parse(include_str!("../data/input.txt"));
    println!("Part 1: {}", map.max_path_len(true));
    println!("Part 2: {}", map.max_path_len(false));
}

#[cfg(test)]
mod test {
    use crate::Map;

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(94, Map::parse(TEST_INPUT).max_path_len(true));
    }

    #[test]
    fn test_part2() {
        assert_eq!(154, Map::parse(TEST_INPUT).max_path_len(false));
    }
}
