use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq)]
enum NodeType {
    Broadcaster,
    FlipFlop,
    Collector,
    Other,
}

fn node_type(name: &str) -> NodeType {
    if name == "broadcaster" {
        NodeType::Broadcaster
    } else if name.starts_with("%") {
        NodeType::FlipFlop
    } else if name.starts_with("&") {
        NodeType::Collector
    } else {
        NodeType::Other
    }
}

fn node_name(name: &str) -> &str {
    if name.chars().nth(0).unwrap().is_ascii_alphabetic() {
        name
    } else {
        &name[1..]
    }
}

fn process(input: &str, iterations: i64, watch: Option<&str>) -> i64 {
    let config = input
        .trim()
        .split("\n")
        .map(|s| {
            let (name, targets) = s.split_once(" -> ").unwrap();
            (
                node_name(name),
                (node_type(name), targets.split(", ").collect::<Vec<_>>()),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut collector_inputs = config
        .iter()
        .filter(|(_, (t, _))| *t == NodeType::Collector)
        .map(|(name, (_, _))| (name.to_string(), HashMap::new()))
        .collect::<HashMap<String, HashMap<String, bool>>>();
    for (name, (_, targets)) in config.iter() {
        for target in targets {
            let n = target.to_string();
            if collector_inputs.contains_key(&n) {
                collector_inputs.get_mut(&n).unwrap().insert(name.to_string(), false);
            }
        }
    }

    let mut flip_flop_states = config
        .iter()
        .filter(|(_, (typ, _))| *typ == NodeType::FlipFlop)
        .map(|(name, (_, _))| (name, false))
        .collect::<HashMap<_, _>>();

    let mut lows = 0;
    let mut highs = 0;
    let mut queue = VecDeque::new();
    for i in 0..iterations {
        queue.push_back(("button", "broadcaster", false));
        while let Some((sender, name, pulse)) = queue.pop_front() {
            if Some(name) == watch && pulse{
                println!("{}: {} -{}-> {}", i+1, sender, if pulse { "high" } else { "low" }, name);
            }
            if pulse == false {
                lows += 1;
            } else {
                highs += 1;
            }
            if let Some((typ, targets)) = config.get(name) {
                match typ {
                    NodeType::Broadcaster => {
                        for t in targets {
                            queue.push_back((name, t, pulse));
                        }
                    },
                    NodeType::FlipFlop => {
                        if pulse == false {
                            let state = flip_flop_states.get_mut(&name).unwrap();
                            *state = !*state;
                            for t in targets {
                                queue.push_back((name, t, *state));
                            }
                        }
                    },
                    NodeType::Collector => {
                        collector_inputs.get_mut(&*name).unwrap().insert(sender.to_string(), pulse);
                        let all_on = collector_inputs.get(name).unwrap().values().all(|b| *b);
                        for t in targets {
                            queue.push_back((name, t, !all_on));
                        }
                    },
                    NodeType::Other => {}
                }
            }
        }
    }
    lows * highs
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part 1: {}", process(input, 1000, None));
    // For part 2, calculate the output by taking the LCM from the loop size for each of the inputs
    // to lcm. It's easiest to do this manually from the printed output.
    process(input, 10_000, Some("zh"));
}

#[cfg(test)]
mod test {
    use crate::process;

    static TEST_INPUT1: &str = include_str!("../data/example1.txt");
    static TEST_INPUT2: &str = include_str!("../data/example2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(32000000, process(TEST_INPUT1, 1000, None));
        assert_eq!(11687500, process(TEST_INPUT2, 1000, None));
    }
}
