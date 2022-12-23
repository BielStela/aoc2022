use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct MoveOrder {
    amount: usize,
    from: usize,
    to: usize,
}

impl MoveOrder {
    pub fn new(move_str: &str) -> Self {
        lazy_static! {
            // ensure that regular expressions are compiled exactly once.
            static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
        }
        let captures = RE
            .captures_iter(move_str)
            .map(|cap| cap[1].parse().unwrap())
            .collect_vec();
        MoveOrder {
            amount: captures[0],
            from: captures[1],
            to: captures[2],
        }
    }
}

struct Cargo {
    stacks: HashMap<usize, Vec<char>>,
}

impl Cargo {
    pub fn new(stacks_str: &str) -> Self {
        let stacks = Cargo::parse_string(stacks_str);
        Cargo { stacks }
    }

    fn parse_string(raw_str: &str) -> HashMap<usize, Vec<char>> {
        unimplemented!();
    }

    pub fn move_crates(&mut self, order: MoveOrder) {
        unimplemented!();
    }
}

pub fn run(input: &str) {
    let (stacks_str, moves_str) = input.split("\n\n").collect_tuple().unwrap();
    // let cargo = Cargo::new(stacks_str);
    for move_order_str in moves_str.split('\n') {
        let move_order = MoveOrder::new(move_order_str);
        println!("{}", move_order.amount);
    }
}
