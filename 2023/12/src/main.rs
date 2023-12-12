use cached::proc_macro::cached;
use std::io::stdin;

#[cached]
fn num_arrangements(row: String, hash_counter: usize, groups: Vec<usize>) -> usize {
    if row.is_empty() {
        if groups.is_empty() || groups.len() == 1 && groups[0] == hash_counter {
            return 1;
        }
        0
    } else if row.starts_with('#') {
        if groups.len() == 0 || groups[0] < (hash_counter + 1) {
            return 0;
        }
        num_arrangements(row[1..].to_string(), hash_counter + 1, groups)
    } else if row.starts_with('.') {
        if hash_counter > 0 {
            if groups.len() == 0 || groups[0] != hash_counter {
                return 0;
            }
            num_arrangements(row[1..].to_string(), 0, groups[1..].to_vec())
        } else {
            num_arrangements(row[1..].to_string(), 0, groups)
        }
    } else if row.starts_with('?') {
        num_arrangements(row.replacen('?', "#", 1), hash_counter, groups.clone())
            + num_arrangements(row.replacen('?', ".", 1), hash_counter, groups)
    } else {
        unreachable!("invalid char");
    }
}

fn parse(line: &str) -> (String, Vec<usize>) {
    let (left, right) = line.split_once(' ').unwrap();
    (
        String::from(left),
        right
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    )
}

fn main() {
    let mut part_1_total = 0;
    let mut part_2_total = 0;

    for line in stdin().lines() {
        let (left, right) = parse(&line.unwrap());
        part_1_total += num_arrangements(left.clone(), 0, right.clone());

        let (left5, right5) = (
            vec![left; 5].join("?"),
            right.iter().cycle().take(right.len() * 5).map(|x| *x).collect(),
        );
        part_2_total += num_arrangements(left5, 0, right5);
    }

    println!("Part 1 total: {}", part_1_total);
    println!("Part 2 total: {}", part_2_total);
}

#[test]
fn test_num_arrangements() {
    assert_eq!(
        1,
        num_arrangements(String::from("???.###"), 0, vec![1, 1, 3])
    );
    assert_eq!(
        4,
        num_arrangements(String::from(".??..??...?##."), 0, vec![1, 1, 3])
    );
    assert_eq!(
        10,
        num_arrangements(String::from("?###????????"), 0, vec![3, 2, 1])
    );
}
