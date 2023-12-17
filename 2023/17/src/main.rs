use core::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
struct Dir(i32, i32);

const INITIAL: Dir = Dir(0, 0);
const EAST: Dir = Dir(0, 1);
const SOUTH: Dir = Dir(1, 0);
const WEST: Dir = Dir(0, -1);
const NORTH: Dir = Dir(-1, 0);

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct Move {
    len: usize,
    row: i32,
    col: i32,
    dir: Dir,
}

fn path_len(s: &str, range_start: usize, range_end: usize) -> usize {
    let blocks: Vec<Vec<u8>> = s
        .trim()
        .split("\n")
        .map(|l| l.bytes().map(|b| b - 0x30).collect())
        .collect();
    let (height, width) = (blocks.len() as i32, blocks[0].len() as i32);

    let mut queue = BinaryHeap::new();
    let mut visited: HashSet<(i32, i32, Dir)> = HashSet::new();

    queue.push(Reverse(Move {
        row: 0,
        col: 0,
        dir: INITIAL,
        len: 0,
    }));

    while let Some(Reverse(Move {
        row: row0,
        col: col0,
        dir: dir0,
        len: len0,
    })) = queue.pop()
    {
        if visited.contains(&(row0, col0, dir0)) {
            continue;
        }
        visited.insert((row0, col0, dir0));

        if row0 == (height - 1) && col0 == (width - 1) {
            return len0;
        }

        let adj: Vec<Dir> = match dir0 {
            EAST => vec![NORTH, SOUTH],
            SOUTH => vec![EAST, WEST],
            WEST => vec![NORTH, SOUTH],
            NORTH => vec![EAST, WEST],
            _ => vec![EAST, SOUTH, WEST, NORTH],
        };
        for dir in adj {
            let mut len = len0;
            let mut row = row0;
            let mut col = col0;

            for n in 0..range_end {
                (row, col) = (row + dir.0, col + dir.1);
                if row < 0 || col < 0 || row >= height || col >= width {
                    break;
                }

                len += blocks[row as usize][col as usize] as usize;

                if n < range_start {
                    continue;
                }

                queue.push(Reverse(Move { row, col, dir, len }));
            }
        }
    }

    unreachable!("No path found");
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part 1: {}", path_len(input, 0, 3));
    println!("Part 2: {}", path_len(input, 3, 10));
}

#[cfg(test)]
mod test {
    use crate::path_len;

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(102, path_len(TEST_INPUT, 0, 3));
    }

    #[test]
    fn test_part2() {
        assert_eq!(94, path_len(TEST_INPUT, 3, 10));
    }
}
