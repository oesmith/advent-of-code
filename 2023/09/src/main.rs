use std::io;

fn calculate_next(seq: &Vec<i32>) -> i32 {
    assert!(seq.len() > 1);

    let mut diffs: Vec<i32> = Vec::with_capacity(seq.len() - 1);
    for i in 0..(seq.len() - 1) {
        diffs.push(seq[i + 1] - seq[i]);
    }

    if diffs.iter().all(|x| *x == 0) {
        *seq.last().unwrap()
    } else {
        seq.last().unwrap() + calculate_next(&diffs)
    }
}

fn main() {
    let mut lines = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!(
        "Part 1 sum: {}",
        lines.iter().map(|l| calculate_next(&l)).sum::<i32>()
    );

    lines.iter_mut().for_each(|l| l.reverse());
    println!(
        "Part 2 sum: {}",
        lines.iter().map(|l| calculate_next(&l)).sum::<i32>()
    );
}

#[test]
fn test_calculate_next() {
    assert_eq!(18, calculate_next(&vec![0, 3, 6, 9, 12, 15]));
    assert_eq!(28, calculate_next(&vec![1, 3, 6, 10, 15, 21]));
    assert_eq!(68, calculate_next(&vec![10, 13, 16, 21, 30, 45]));
}
