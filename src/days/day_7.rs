use util::files::read_file_to_string;

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;

pub fn main() {
    println!("-- Running Day 7 --");
    let input = &read_file_to_string("src/days/input/day_7.txt");
    println!("{}", input);
    let root = root_node_from_input(input);
    println!("{}", root);
}

fn root_node_from_input(input: &str) -> TowerNode {
    let mut holders = HashMap::new();

    let x = TowerNode {
        passengers: Vec::new(),
        weight: 42u32,
        name: "Andrew"
    };

    holders.insert("", x);

    holders.remove("").unwrap()
}

struct TowerNode<'a> {
    passengers: Vec<TowerNode<'a>>,
    weight: u32,
    name: &'a str
}

impl<'a> Display for TowerNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.passengers.len() > 0 {
            write!(f, "{} ({}) -> has passengers", self.name, self.weight)
        } else {
            write!(f, "{} ({})", self.name, self.weight)
        }
    }

}