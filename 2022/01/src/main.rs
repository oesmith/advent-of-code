use std::env;
use std::io;

fn main() {
    let mut push_next_value = true;
    let mut elves = vec![0];
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            push_next_value = true;
        } else {
            let n: i32 = line_str.parse().unwrap();
            if push_next_value {
                push_next_value = false;
                elves.push(n)
            } else {
                *(elves.last_mut().unwrap()) += n
            }
        }
    }

    if env::args().any(|x| x == "top3") {
        elves.sort();
        elves.reverse();
        let top3 = elves.iter().take(3).sum::<i32>();
        print!("Top 3 elves are carrying {} calories", top3);
    } else {
        let max = elves.iter().max().unwrap();
        print!("Maximum calories: {}", max);
    }
}
