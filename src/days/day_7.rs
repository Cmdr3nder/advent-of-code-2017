use util::files::read_file_to_string;

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;

pub fn main() {
    println!("-- Running Day 7 --");
    let input = &read_file_to_string("src/days/input/day_7.txt");
    let root = root_node_from_input(input);
    println!("{}", root);
}

fn root_node_from_input<'a>(input: &'a str) -> TowerNode<'a> {
    let mut indexed_lines: HashMap<&str, &str> = input.lines().map(|l| {
        let line = l.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        (parts[0], line)
    }).collect();

    let mut root_name: &str = "";

    for name in indexed_lines.keys() {
        if substring_count(input, name) == 1 {
            root_name = name;
            break;
        }
    }

    match indexed_lines.remove(root_name) {
        Some(root_line) => {
            generate_node(root_line, &indexed_lines)
        },
        None => panic!("Couldn't figure out root line.")
    }
}

fn substring_count(full_string: &str, substring: &str) -> usize {
    full_string.split(substring).count() - 1
}

#[test]
fn substring_count_test() {
    assert_eq!(0, substring_count("Hello, World", "42"));
    assert_eq!(1, substring_count("42 Hello, World", "42"));
    assert_eq!(2, substring_count("42 Hello, World 42", "42"));
    assert_eq!(3, substring_count("42 Hello,42 World 42", "42"));
}

fn generate_node<'a>(current_line: &'a str, all_lines: &HashMap<&'a str, &'a str>) -> TowerNode<'a> {
    let mut parts: Vec<&str> = current_line.split_whitespace().collect();
    let name = parts[0];
    let weight = parts[1].trim_matches(|c| {
        c == '(' || c == ')'
    }).parse::<u32>().unwrap();

    let passengers: Vec<&str> = parts.split_off(2)
                                              .into_iter()
                                              .filter(|&p| p != "->")
                                              .map(|p| {
                                                  *all_lines.get(p.trim_matches(',')).unwrap()
                                              }).collect();

    let mut built_passengers: Vec<TowerNode<'a>> = Vec::new();

    for passenger in passengers {
        built_passengers.push(generate_node(passenger, &all_lines));
    }

    TowerNode {
        name,
        weight,
        passengers: built_passengers
    }
}

struct TowerNode<'a> {
    passengers: Vec<TowerNode<'a>>,
    weight: u32,
    name: &'a str
}

impl<'a> Display for TowerNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.passengers.len() > 0 {
            let mut passengers_str = String::new();
            for passenger_index in 0..(self.passengers.len() - 1) {
                passengers_str.push_str(self.passengers[passenger_index].name);
                passengers_str.push_str(", ");
            }
            passengers_str.push_str(self.passengers[self.passengers.len() - 1].name);

            write!(f, "{} ({}) -> {}", self.name, self.weight, passengers_str)
        } else {
            write!(f, "{} ({})", self.name, self.weight)
        }
    }
}