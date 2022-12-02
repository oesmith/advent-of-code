use std::collections::HashMap;
use std::env;
use std::io;


fn main() {
    let mut scores = HashMap::new();

    // Rock = 1
    // Paper = 2
    // Scissors = 3
    if env::args().any(|x| x == "winlose") {
        scores.insert(String::from("A X"), 0 + 3);
        scores.insert(String::from("A Y"), 3 + 1);
        scores.insert(String::from("A Z"), 6 + 2);
        scores.insert(String::from("B X"), 0 + 1);
        scores.insert(String::from("B Y"), 3 + 2);
        scores.insert(String::from("B Z"), 6 + 3);
        scores.insert(String::from("C X"), 0 + 2);
        scores.insert(String::from("C Y"), 3 + 3);
        scores.insert(String::from("C Z"), 6 + 1);
    } else {
        scores.insert(String::from("A X"), 3 + 1);
        scores.insert(String::from("A Y"), 6 + 2);
        scores.insert(String::from("A Z"), 0 + 3);
        scores.insert(String::from("B X"), 0 + 1);
        scores.insert(String::from("B Y"), 3 + 2);
        scores.insert(String::from("B Z"), 6 + 3);
        scores.insert(String::from("C X"), 6 + 1);
        scores.insert(String::from("C Y"), 0 + 2);
        scores.insert(String::from("C Z"), 3 + 3);
    }

    let mut total = 0;
    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();
        total += scores.get(&line_str).unwrap();
    }
    println!("Total score: {}", total);
}
