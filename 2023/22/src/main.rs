use core::cmp::Reverse;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap());

#[derive(Hash, Eq, PartialEq, Clone)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone)]
struct Brick {
    z0: i32,
    z1: i32,
    blocks: Vec<Block>,
}

impl<'a> Brick {
    fn parse(s: &str) -> Brick {
        let c = RE
            .captures(s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let x0 = c[0].min(c[3]);
        let x1 = c[0].max(c[3]);
        let y0 = c[1].min(c[4]);
        let y1 = c[1].max(c[4]);
        let z0 = c[2].min(c[5]);
        let z1 = c[2].max(c[5]);
        let (dx, dy, dz) = (x1 - x0 + 1, y1 - y0 + 1, z1 - z0 + 1);
        let n = dx.max(dy).max(dz);
        let blocks = (0..n)
            .map(|i| Block {
                x: x0 + (i * dx / n),
                y: y0 + (i * dy / n),
                z: z0 + (i * dz / n),
            })
            .collect();
        Brick { blocks, z0, z1 }
    }

    fn bottom(&'a self) -> Vec<&'a Block> {
        self.blocks.iter().filter(|b| b.z == self.z0).collect()
    }

    fn top(&'a self) -> Vec<&'a Block> {
        self.blocks.iter().filter(|b| b.z == self.z1).collect()
    }

    fn drop(&mut self, n: i32) {
        self.z0 -= n;
        self.z1 -= n;
        for b in &mut self.blocks {
            b.z -= n;
        }
    }
}

struct Stack {
    count: usize,
    above: HashMap<usize, HashSet<usize>>,
    below: HashMap<usize, HashSet<usize>>,
}

impl Stack {
    fn create(input: &str) -> Stack {
        let mut bricks = input
            .trim()
            .split("\n")
            .map(|s| Brick::parse(s))
            .collect::<Vec<_>>();
        let mut topo: HashMap<(i32, i32), i32> = HashMap::new();
        bricks.sort_by_key(|b| b.z0);
        for i in 0..bricks.len() {
            let dz = bricks[i]
                .bottom()
                .iter()
                .map(|b| b.z - topo.get(&(b.x, b.y)).unwrap_or(&0) - 1)
                .min()
                .unwrap();
            bricks[i].drop(dz);
            bricks[i].blocks.iter().for_each(|b| {
                topo.insert((b.x, b.y), b.z);
            });
        }
        bricks.sort_by_key(|b| b.z0);
        let mut voxels: HashMap<(i32, i32, i32), usize> = HashMap::new();
        for i in 0..bricks.len() {
            bricks[i].blocks.iter().for_each(|b| {
                assert!(!voxels.contains_key(&(b.x, b.y, b.z)));
                voxels.insert((b.x, b.y, b.z), i);
            });
        }
        let mut above: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut below: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..bricks.len() {
            let up = bricks[i]
                .top()
                .iter()
                .flat_map(|b| voxels.get(&(b.x, b.y, b.z + 1)))
                .map(|j| j.clone())
                .collect::<HashSet<_>>();
            above.insert(i, up);
            let down = bricks[i]
                .bottom()
                .iter()
                .flat_map(|b| voxels.get(&(b.x, b.y, b.z - 1)))
                .map(|j| j.clone())
                .collect::<HashSet<_>>();
            below.insert(i, down);
        }
        Stack {
            count: bricks.len(),
            above,
            below,
        }
    }

    fn num_can_disintegrate(&self) -> usize {
        (0..self.count)
            .filter(|i| {
                self.above
                    .get(i)
                    .unwrap()
                    .iter()
                    .all(|j| self.below.get(j).unwrap().len() > 1)
            })
            .count()
    }

    fn num_fall(&self, victim: usize) -> usize {
        let mut fallen: HashSet<usize> = HashSet::new();
        let mut queue = BinaryHeap::new();

        fallen.insert(victim);
        queue.push(Reverse(victim));
        while let Some(Reverse(i)) = queue.pop() {
            for j in self.above.get(&i).unwrap() {
                if self
                    .below
                    .get(j)
                    .unwrap()
                    .iter()
                    .all(|k| fallen.contains(k))
                {
                    fallen.insert(*j);
                    queue.push(Reverse(*j));
                }
            }
        }

        fallen.len() - 1
    }

    fn max_fall_after_single_disintegration(&self) -> usize {
        (0..self.count)
            .filter(|i| {
                self.above
                    .get(i)
                    .unwrap()
                    .iter()
                    .any(|j| self.below.get(j).unwrap().len() == 1)
            })
            .map(|i| self.num_fall(i))
            .sum()
    }
}

fn main() {
    let stack = Stack::create(include_str!("../data/input.txt"));
    println!("Part 1: {}", stack.num_can_disintegrate());
    println!("Part 2: {}", stack.max_fall_after_single_disintegration());
}

#[cfg(test)]
mod test {
    use crate::Stack;

    const TEST_INPUT: &str = include_str!("../data/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(5, Stack::create(TEST_INPUT).num_can_disintegrate());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            7,
            Stack::create(TEST_INPUT).max_fall_after_single_disintegration()
        );
    }
}
