use regex::Regex;
use std::env;
use std::io;

fn main() {
    let multi_move = env::args().any(|x| x == "multi");

    let mut rows = Vec::<String>::new();
    let mut moves = Vec::<String>::new();

    let mut add_to_rows = true;
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            add_to_rows = false;
        } else if add_to_rows {
            rows.push(line_str);
        } else {
            moves.push(line_str);
        }
    }

    let mut stacks = Vec::<Vec<u8>>::new();

    while !rows.is_empty() {
        let row = rows.pop();
        for (index, &chr) in row.unwrap().as_bytes().iter().enumerate() {
            if index % 4 == 1 && chr.is_ascii_alphanumeric() {
                let stack_index = index / 4;
                if stack_index >= stacks.len() {
                    stacks.push(Vec::from([chr]));
                } else {
                    stacks[stack_index].push(chr);
                }
            }
        }
    }

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for mov in moves {
        let caps = re.captures(&mov).unwrap();
        let count: usize = caps[1].parse().unwrap();
        let src: usize = caps[2].parse::<usize>().unwrap() - 1;
        let dst: usize = caps[3].parse::<usize>().unwrap() - 1;

        let split_index = stacks[src].len() - count;
        let mut crates = stacks[src].split_off(split_index);
        if !multi_move {
            crates.reverse();
        }
        stacks[dst].extend(crates);
    }

    print!(
        "{}\n",
        String::from_utf8(stacks.iter().map(|s| *s.last().unwrap()).collect()).unwrap()
    );
}
