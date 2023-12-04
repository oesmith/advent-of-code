use std::collections::HashMap;
use std::io;

fn main() {
    let buf: Vec<Vec<u8>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    let width = buf[0].len();

    let mut sum_of_part_numbers: i32 = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for y in 0..buf.len() {
        let el = &buf[y];
        debug_assert_eq!(width, el.len());
        let mut x = 0;
        while x < el.len() {
            if el[x].is_ascii_digit() {
                // Read the entire number.
                let mut num: i32 = 0;
                let x1 = if x > 0 { x - 1 } else { x };
                while x < el.len() && el[x].is_ascii_digit() {
                    num = num * 10 + (el[x] - b'0') as i32;
                    x += 1;
                }
                // Scan the bounding box around the number to see if it contains any symbols.
                let y1 = if y > 0 { y - 1 } else { y };
                let y2 = buf.len().min(y + 2);
                let x2 = el.len().min(x + 1);
                'outer: for v in y1..y2 {
                    for u in x1..x2 {
                        let sym = buf[v][u];
                        if sym != b'.' && !sym.is_ascii_digit() {
                            if sym == b'*' {
                                gears.entry((u, v)).or_insert(Vec::new()).push(num);
                            }
                            sum_of_part_numbers += num;
                            break 'outer;
                        }
                    }
                }
            }
            x += 1;
        }
    }

    let sum_of_gear_ratios = gears
        .into_values()
        .filter(|g| g.len() == 2)
        .fold(0, |s, g| s + g[0] * g[1]);

    println!("Sum of part numbers: {}", sum_of_part_numbers);
    println!("Sum of gear ratios: {}", sum_of_gear_ratios);
}
