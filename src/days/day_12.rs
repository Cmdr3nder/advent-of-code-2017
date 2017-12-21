use util::files::read_file_to_string;

use std::collections::HashSet;

pub fn main() {
    println!("-- Running Day 12 --");
    let input = &read_file_to_string("src/days/input/day_12.txt");
    let pipes = parse_pipes(input);
    let attached = find_attached(&pipes, 0);
    println!("Nodes attached to '0' = {}", attached.len());
    println!("Group count = {}", count_groups(&pipes));
}

fn parse_pipes(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split("<->").collect();
        parts[1].split(",").map(|pipe_str| {
            pipe_str.trim()
        }).filter(|pipe_str| {
            !pipe_str.is_empty()
        }).map(|pipe_str| {
            pipe_str.parse::<usize>().unwrap()
        }).collect()
    }).collect()
}

fn find_attached(pipes: &Vec<Vec<usize>>, to: usize) -> HashSet<usize> {
    let mut attached = HashSet::new();
    let mut prev_len = 0;

    attached.insert(to);

    while prev_len != attached.len() {
        prev_len = attached.len();

        for index in attached.clone() {
            for new_index in pipes[index].clone() {
                attached.insert(new_index);
            }
        }
    }

    attached
}

fn count_groups(pipes: &Vec<Vec<usize>>) -> usize {
    let mut unique_groups: Vec<HashSet<usize>> = Vec::new();

    for program_index in 0..pipes.len() {
        if !contained_in_any_group(&unique_groups, program_index) {
            unique_groups.push(find_attached(pipes, program_index));
        }
    }

    unique_groups.len()
}

fn contained_in_any_group(unique_groups: &Vec<HashSet<usize>>, program: usize) -> bool {
    for group in unique_groups {
        if group.contains(&program) {
            return true
        }
    }

    false
}