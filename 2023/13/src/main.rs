use std::io::stdin;

struct Input {
    rows: Vec<u64>,
    rev_rows: Vec<u64>,
    cols: Vec<u64>,
    rev_cols: Vec<u64>,
}

impl Input {
    fn parse(row_strs: &Vec<String>) -> Input {
        // Turn each row into a bitfield.
        let rows: Vec<u64> = row_strs
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| if c == '#' { 1 as u64 } else { 0 as u64 })
                    .reduce(|a, b| (a << 1) + b)
                    .unwrap()
            })
            .collect();

        // Create a copy of rows in reverse order (to save duplication of effort later).
        let mut rev_rows = rows.clone();
        rev_rows.reverse();

        // Turn each column into a bitfield.
        let width = row_strs[0].len();
        let cols: Vec<u64> = (0..width)
            .into_iter()
            .map(|i| {
                rows.iter()
                    .map(|r| if r & (1 << width - 1 - i) != 0 { 1 } else { 0 })
                    .fold(0, |a, b| (a << 1) + b)
            })
            .collect();

        // ... and reversed.
        let mut rev_cols = cols.clone();
        rev_cols.reverse();

        Input {
            rows,
            rev_rows,
            cols,
            rev_cols,
        }
    }

    fn reflections(&self) -> usize {
        100 * find_reflection(&self.rows, &self.rev_rows, |a, b| a == b)
            + find_reflection(&self.cols, &self.rev_cols, |a, b| a == b)
    }

    fn smudgy_reflections(&self) -> usize {
        100 * find_reflection(&self.rows, &self.rev_rows, |a, b| smudge_comparator(a, b))
            + find_reflection(&self.cols, &self.rev_cols, |a, b| smudge_comparator(a, b))
    }
}

fn smudge_comparator(a: &[u64], b: &[u64]) -> bool {
    assert_eq!(a.len(), b.len());
    // Count bits that differ between the two slices, and return true if the total is 1.
    a.iter()
        .zip(b)
        .map(|(u, v)| (u ^ v).count_ones())
        .sum::<u32>()
        == 1
}

fn find_reflection(
    fwd: &Vec<u64>,
    rev: &Vec<u64>,
    comparator: fn(a: &[u64], b: &[u64]) -> bool,
) -> usize {
    assert_eq!(fwd.len(), rev.len());
    for i in 1..fwd.len() {
        let l = i.min(fwd.len() - i);
        assert!(l > 0);
        let fwd_slice = &fwd[(i - l)..i];
        let rev_slice = &rev[(fwd.len() - i - l)..(fwd.len() - i)];
        if comparator(fwd_slice, rev_slice) {
            return i;
        }
    }
    0
}

fn parse() -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];
    let mut current: Vec<String> = vec![];
    for line in stdin().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            inputs.push(Input::parse(&current));
            current.clear();
        } else {
            current.push(l.clone());
        }
    }
    if !current.is_empty() {
        inputs.push(Input::parse(&current));
    }
    inputs
}

fn main() {
    let inputs = parse();
    let part_1_total: usize = inputs.iter().map(|i| i.reflections()).sum();
    let part_2_total: usize = inputs.iter().map(|i| i.smudgy_reflections()).sum();
    println!("Part 1 total: {}", part_1_total);
    println!("Part 2 total: {}", part_2_total);
}

#[test]
fn test_row_reflections() {
    let input = Input::parse(&vec![
        "#.##..##.".to_string(),
        "..#.##.#.".to_string(),
        "##......#".to_string(),
        "##......#".to_string(),
        "..#.##.#.".to_string(),
        "..##..##.".to_string(),
        "#.#.##.#.".to_string(),
    ]);
    assert_eq!(5, input.reflections());
}

#[test]
fn test_col_reflections() {
    let input = Input::parse(&vec![
        "#...##..#".to_string(),
        "#....#..#".to_string(),
        "..##..###".to_string(),
        "#####.##.".to_string(),
        "#####.##.".to_string(),
        "..##..###".to_string(),
        "#....#..#".to_string(),
    ]);
    assert_eq!(400, input.reflections());
}

#[test]
fn test_col_smudgy_reflections() {
    let input = Input::parse(&vec![
        "#.##..##.".to_string(),
        "..#.##.#.".to_string(),
        "##......#".to_string(),
        "##......#".to_string(),
        "..#.##.#.".to_string(),
        "..##..##.".to_string(),
        "#.#.##.#.".to_string(),
    ]);
    assert_eq!(300, input.smudgy_reflections());
}

#[test]
fn test_parse() {
    let input = Input::parse(&vec![
        "#.#".to_string(),
        ".#.".to_string(),
        "##.".to_string(),
    ]);
    assert_eq!(
        vec![0b101, 0b11, 0b100],
        input.cols
    );
    assert_eq!(
        vec![0b100, 0b11, 0b101],
        input.rev_cols
    );
    assert_eq!(
        vec![0b101, 0b10, 0b110],
        input.rows
    );
    assert_eq!(
        vec![0b110, 0b10, 0b101],
        input.rev_rows
    );
}
