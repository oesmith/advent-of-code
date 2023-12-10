use std::collections::VecDeque;
use std::io;

struct Field {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Field {
    fn parse() -> Field {
        let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
        let width = lines[0].len();
        let height = lines.len();
        let map: Vec<char> = lines.join("").chars().collect();
        assert_eq!(width * height, map.len());
        Field { map, width, height }
    }

    fn visit(&self) -> Vec<usize> {
        let mut visited: Vec<usize> = vec![0; self.map.len()];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        let start = self.start_pos();
        for p in self.neighbours(start) {
            if self.connections(p).contains(&start) {
                visited[self.offset(p)] = 1;
                queue.push_back(p);
            }
        }

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            let l = visited[self.offset(p)];
            for pp in self.connections(p) {
                if visited[self.offset(pp)] == 0 {
                    visited[self.offset(pp)] = l + 1;
                    queue.push_back(pp);
                }
            }
        }

        visited
    }

    fn max_path_len(&self) -> usize {
        *self.visit().iter().max().unwrap()
    }

    fn num_enclosed(&self) -> usize {
        let visited = self.visit();

        let mut count = 0;

        let mut is_inside = false;
        let mut last_seen_corner = '.';

        for y in 0..self.height {
            for x in 0..self.width {
                let p = (x, y);
                let mut tile = self.get(p);
                if tile == 'S' {
                    tile = self.infer_tile(p);
                } else if visited[self.offset(p)] == 0 {
                    tile = '.'; // Treat disconnected pipes as spaces.
                }
                match tile {
                    '-' => (),
                    '|' => is_inside = !is_inside,
                    'F' | 'L' => last_seen_corner = tile,
                    'J' => {
                        if last_seen_corner == 'F' {
                            is_inside = !is_inside
                        }
                    }
                    '7' => {
                        if last_seen_corner == 'L' {
                            is_inside = !is_inside
                        }
                    }
                    '.' => {
                        if is_inside {
                            count += 1
                        }
                    }
                    _ => unreachable!("Invalid tile"),
                }
            }
        }

        count
    }

    fn offset(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn start_pos(&self) -> (usize, usize) {
        let index = self
            .map
            .iter()
            .position(|x| *x == 'S')
            .expect("Missing start point");
        (index % self.width, index / self.width)
    }

    fn get(&self, (x, y): (usize, usize)) -> char {
        assert!(x < self.width);
        assert!(y < self.height);

        self.map[y * self.width + x]
    }

    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        assert!(x < self.width);
        assert!(y < self.height);

        let mut candidates: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            candidates.push((x - 1, y))
        }
        if y > 0 {
            candidates.push((x, y - 1))
        }
        if x < self.width - 1 {
            candidates.push((x + 1, y))
        }
        if y < self.height - 1 {
            candidates.push((x, y + 1))
        }
        candidates
    }

    fn connections(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        assert!(x < self.width);
        assert!(y < self.height);

        let tile = self.get((x, y));
        match tile {
            'F' => vec![(x + 1, y), (x, y + 1)],
            'L' => vec![(x + 1, y), (x, y - 1)],
            '7' => vec![(x - 1, y), (x, y + 1)],
            'J' => vec![(x - 1, y), (x, y - 1)],
            '-' => vec![(x - 1, y), (x + 1, y)],
            '|' => vec![(x, y - 1), (x, y + 1)],
            _ => vec![],
        }
    }

    // Work out what kind of tile is at a given position -- this is useful for turning an 'S' tile
    // into its actual type.
    fn infer_tile(&self, p: (usize, usize)) -> char {
        let neighbours = self.neighbours(p);
        let mut adj = neighbours
            .iter()
            .filter(|n| self.connections(**n).contains(&p))
            .collect::<Vec<_>>();
        adj.sort();
        assert_eq!(2, adj.len()); // Tiles in the loop *must* have two connections.
        let ((_, y), (x1, y1), (x2, y2)) = (p, adj[0], adj[1]);
        if x1 == x2 {
            '|'
        } else if y1 == y2 {
            '-'
        } else if y1 < &y {
            'L'
        } else if y1 > &y {
            'F'
        } else if y2 < &y {
            'J'
        } else if y2 > &y {
            '7'
        } else {
            unreachable!("Invalid start")
        }
    }
}

fn main() {
    let f = Field::parse();

    println!("Max. path len.: {}", f.max_path_len());
    println!("Num. enclosed: {}", f.num_enclosed());
}
