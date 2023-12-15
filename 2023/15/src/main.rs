use std::io::stdin;

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |a, b| (a.wrapping_add(b)).wrapping_mul(17))
}

fn part1(input: &str) -> u32 {
    input.trim().split(",").map(|s| hash(s) as u32).sum()
}

fn part2(input: &str) -> u32 {
    let mut hashmap: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    'a: for s in input.trim().split(",") {
        if s.ends_with('-') {
            let k = &s[..s.len() - 1];
            let h = hash(k) as usize;
            for i in 0..hashmap[h].len() {
                if hashmap[h][i].0 == k {
                    hashmap[h].remove(i);
                    break;
                }
            }
        } else {
            let (k, v) = s.split_once('=').unwrap();
            let n = v.parse::<u32>().unwrap();
            let h = hash(k) as usize;
            for i in 0..hashmap[h].len() {
                if hashmap[h][i].0 == k {
                    hashmap[h][i].1 = n;
                    continue 'a;
                }
            }
            hashmap[h].push((k.to_string(), n));
        }
    }
    hashmap
        .iter()
        .zip(1..257)
        .map(|(b, i)| {
            b.iter()
                .zip(1..b.len() + 1)
                .map(|((_, n), j)| n * j as u32)
                .sum::<u32>()
                * i as u32
        })
        .sum::<u32>()
}

fn main() {
    let mut input = String::new();
    assert!(stdin().read_line(&mut input).is_ok());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(1320, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(145, part2(TEST_INPUT));
    }
}
