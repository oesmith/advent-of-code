use std::env;
use std::io::{self, Read};

enum Operation {
    ADD,
    MUL,
    SQ,
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    operand: usize,
    test_divisible_by: usize,
    if_true: usize,
    if_false: usize,
    num_inspections: usize,
}

fn main() {
    let long = env::args().any(|x| x == "long");
    let num_rounds = if long { 10_000 } else { 20 };

    let mut monkeys = parse_monkeys();

    // The product of all of the test_divisible_by values. Used to avoid worry
    // levels overflowing, while still maintaining the ability to test divisibility.
    let test_product = monkeys
        .iter()
        .map(|m| m.test_divisible_by)
        .reduce(|a, e| a * e)
        .unwrap();

    for _ in 0..num_rounds {
        for m in 0..monkeys.len() {
            for i in 0..monkeys[m].items.len() {
                let mut worry = monkeys[m].items[i];
                worry = match monkeys[m].operation {
                    Operation::ADD => worry + monkeys[m].operand,
                    Operation::MUL => worry * monkeys[m].operand,
                    Operation::SQ => worry * worry,
                };
                worry = if long {
                    worry % test_product
                } else {
                    worry / 3
                };
                let next = if worry % monkeys[m].test_divisible_by == 0 {
                    monkeys[m].if_true
                } else {
                    monkeys[m].if_false
                };
                monkeys[next].items.push(worry);
                monkeys[m].num_inspections += 1;
            }
            monkeys[m].items.clear();
        }
    }

    let mut inspection_counts: Vec<usize> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspection_counts.sort();
    inspection_counts.reverse();
    print!(
        "Monkey business: {}\n",
        inspection_counts[0] * inspection_counts[1]
    );
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let mut monkeys = Vec::new();
    let groups = data.split("\n\n");
    for group in groups {
        let monkey = parse_monkey(group.trim());
        assert!(monkey.id == monkeys.len());
        monkeys.push(monkey);
    }

    return monkeys;
}

fn parse_monkey(s: &str) -> Monkey {
    let mut lines = s.trim().split('\n');
    let id = parse_id(lines.next().unwrap());
    let items = parse_items(lines.next().unwrap());
    let (operation, operand) = parse_operation(lines.next().unwrap());
    let test_divisible_by = parse_test_divisible_by(lines.next().unwrap());
    let if_true = parse_if_true(lines.next().unwrap());
    let if_false = parse_if_false(lines.next().unwrap());
    let num_inspections = 0;
    return Monkey {
        id,
        items,
        operation,
        operand,
        test_divisible_by,
        if_true,
        if_false,
        num_inspections,
    };
}

fn parse_id(s: &str) -> usize {
    s.get(7..s.len() - 1).unwrap().parse().unwrap()
}

fn parse_items(s: &str) -> Vec<usize> {
    s.get(18..)
        .unwrap()
        .split(", ")
        .map(|i| i.parse().unwrap())
        .collect()
}

fn parse_operation(s: &str) -> (Operation, usize) {
    if s.ends_with("old * old") {
        return (Operation::SQ, 0);
    }
    let oper = match s.chars().nth(23).unwrap() {
        '+' => Operation::ADD,
        '*' => Operation::MUL,
        _ => panic!(),
    };
    let operand = s.get(25..).unwrap().parse().unwrap();
    return (oper, operand);
}

fn parse_test_divisible_by(s: &str) -> usize {
    s.get(21..).unwrap().parse().unwrap()
}

fn parse_if_true(s: &str) -> usize {
    s.get(29..).unwrap().parse().unwrap()
}

fn parse_if_false(s: &str) -> usize {
    s.get(30..).unwrap().parse().unwrap()
}
