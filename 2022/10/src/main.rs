use std::env;
use std::io;

fn main() {
    if env::args().any(|x| x == "render") {
        render();
    } else {
        total();
    }
}

fn render() {
    let mut counter: i32 = 0;
    let mut x = 1;
    let instr = instructions();
    for (n, dx) in instr {
        for _ in 0..n {
            if (x-(counter % 40)).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            counter += 1;
            if counter % 40 == 0 {
                print!("\n");
            }
        }
        x += dx;
    }
}

fn total() {
    let mut total = 0;
    let mut counter = 0;
    let mut next = 20;
    let mut x = 1;
    let instr = instructions();
    for (n, dx) in instr {
        counter += n;
        if counter >= next {
            total += next * x;
            next += 40;
        }
        x += dx;
    }
    print!("Total: {}\n", total);
}

fn instructions() -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();
        ret.push(match line_str.get(..4) {
            Some("noop") => (1, 0),
            Some("addx") => (2, line_str.get(5..).unwrap().parse().unwrap()),
            _ => panic!(),
        });
    }
    return ret;
}
