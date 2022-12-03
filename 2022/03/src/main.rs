use std::assert;
use std::collections::HashSet;
use std::env;
use std::io;

fn main() {
    if env::args().any(|x| x == "badges") {
        badges();
    } else {
        common();
    }
}

fn badges() {
    let lines = io::stdin().lines();
    let mut total: i32 = 0;
    let mut group_bytes = HashSet::<u8>::new();
    for (index, line) in lines.enumerate() {
        let line = line.unwrap();
        if index % 3 == 0 {
            for &element in line.as_bytes() {
                group_bytes.insert(element);
            }
        } else {
            let mut bytes = HashSet::<u8>::new();
            for &element in line.as_bytes() {
                bytes.insert(element);
            }
            group_bytes.retain(|x| bytes.contains(x));
        }
        if index % 3 == 2 {
            for &element in &group_bytes {
                total += priority(element);
            }
            group_bytes.clear();
        }
    }
    print!("Total {}", total);
}

fn common() {
    let lines = io::stdin().lines();
    let mut total: i32 = 0;
    for line in lines {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        assert!(bytes.len() % 2 == 0);
        let half_way = bytes.len() / 2;
        let mut first = HashSet::new();
        for element in bytes[..half_way].iter() {
            first.insert(element);
        }
        let mut second = HashSet::new();
        for element in bytes[half_way..].iter() {
            second.insert(element);
        }
        for &element in first.intersection(&second) {
            total += priority(*element);
        }
    }
    print!("Total {}", total);
}

fn priority(element: u8) -> i32 {
    if element >= 0x61 {
        return i32::from(element - 0x60);
    }
    return i32::from(element - 0x41 + 27);
}
