use std::env;
use std::io;

enum Mode {
    Contains,
    Overlap,
}

fn main() {
    let mode = if env::args().any(|x| x == "overlap") {
        Mode::Overlap
    } else {
        Mode::Contains
    };
    let lines = io::stdin().lines();
    let mut count = 0;
    for line in lines {
        let line_str = line.unwrap();
        let (first, second) = line_str.split_once(',').unwrap();
        if test(&mode, range(first), range(second)) {
            count += 1;
        }
    }
    print!("Count: {}\n", count);
}

fn range(s: &str) -> (i32, i32) {
    let (start, end) = s.split_once('-').unwrap();
    return (start.parse().unwrap(), end.parse().unwrap());
}

fn test(mode: &Mode, first: (i32, i32), second: (i32, i32)) -> bool {
    let (a, b) = first;
    let (c, d) = second;
    return match mode {
        Mode::Contains => (a <= c && b >= d) || (c <= a && d >= b),
        Mode::Overlap => !(b < c || a > d),
    };
}
