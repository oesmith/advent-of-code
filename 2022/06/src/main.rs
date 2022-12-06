use std::assert;
use std::env;
use std::io::{self, Read};

fn main() {
    let n = if env::args().any(|x| x == "message") {
        14
    } else {
        4
    };

    let mut message = String::new();
    io::stdin().read_to_string(&mut message).unwrap();
    let message_bytes = message.as_bytes();

    assert!(message.len() > n);

    let mut chrs = vec![0; 256];
    let mut n_dups = 0;
    for i in 0..n {
        let c = message_bytes[i] as usize;
        if chrs[c] > 0 {
            n_dups += 1;
        }
        chrs[c] += 1;
    }

    if n_dups == 0 {
        print!("{}\n", n);
        return;
    }

    for i in n..message_bytes.len() {
        let last = message_bytes[i] as usize;
        if chrs[last] > 0 {
            n_dups += 1;
        }
        chrs[last] += 1;

        let first = message_bytes[i - n] as usize;
        if chrs[first] > 1 {
            n_dups -= 1;
        }
        chrs[first] -= 1;

        if n_dups == 0 {
            print!("{}\n", i + 1);
            return;
        }
    }

    print!("Not found\n");
}
