use util::files::read_file_to_string;

use std::ascii::AsciiExt;

pub fn main() {
    println!("-- Running Day 10 --");
    let input = &read_file_to_string("src/days/input/day_10.txt");

    println!("Multiple of first couple is {}", non_challenge(input));
    println!("Challenge hash is {}", challenge(input));
}

fn non_challenge(input: &str) -> usize {
    let lengths = parse_lengths_from_list(input);
    let mut strand = initialize_strand(256);

    execute_lengths(&mut strand, lengths, 1);

    strand[0] * strand[1]
}

fn challenge(input: &str) -> String {
    let lengths = parse_lengths_from_chars(input, &[17, 31, 73, 47, 23]);
    let mut strand = initialize_strand(256);

    execute_lengths(&mut strand, lengths, 64);

    strand = collapse_strand(&strand, 16);

    format_strand(&strand)
}

#[test]
fn test_challenge() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", challenge(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", challenge("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", challenge("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", challenge("1,2,4"));
}

fn format_strand(strand: &[usize]) -> String {
    let mut result = String::new();

    for index in 0..strand.len() {
        result = result + format!("{:02x}", strand[index]).as_str();
    }

    result
}

#[test]
fn test_format_strand() {
    assert_eq!("0a0b", format_strand(&[10, 11]));
}

fn collapse_strand(strand: &[usize], groups: usize) -> Vec<usize> {
    if (strand.len() % groups) != 0 {
        panic!("Poor group size ({} % {}) != 0", strand.len(), groups);
    }

    let mut dense_strand = Vec::new();
    let group_size = strand.len() / groups;

    for group_index in 0..groups {
        let mut dense_value = strand[group_size * group_index];

        for index in 1..group_size {
            dense_value = dense_value ^ strand[(group_size * group_index) + index];
        }

        dense_strand.push(dense_value);
    }

    dense_strand
}

fn parse_lengths_from_list(input: &str) -> Vec<usize> {
    input.split(',').map(|s| {
        s.trim().parse::<usize>().unwrap()
    }).collect()
}

fn parse_lengths_from_chars(input: &str, extras: &[usize]) -> Vec<usize> {
    let mut char_buff = [0; 1];
    let mut lengths: Vec<usize> = input.chars().map(|ch| {
        if ch.is_ascii() {
            ch.encode_utf8(&mut char_buff);
            char_buff[0] as usize
        } else {
            panic!("Non-ascii input detected {}", ch);
        }
    }).collect();
    
    for extra in extras {
        lengths.push(*extra);
    }

    lengths
}

fn execute_lengths(strand: &mut [usize], lengths: Vec<usize>, iterations: usize) {
    let mut current_index = 0;
    let mut skip_size = 0;

    for _ in 0..iterations {
        for length in lengths.clone() {
            reverse_sub_slice(strand, current_index, length);

            current_index = (current_index + length + skip_size) % strand.len();
            skip_size = skip_size + 1;
        }
    }
}

fn initialize_strand(size: usize) -> Vec<usize> {
    let mut strand = Vec::new();

    for x in 0..size {
        strand.push(x);
    }

    strand
}

fn reverse_sub_slice(slice: &mut [usize], start: usize, count: usize) {
    for swap_counter in 0..(count / 2) {
        let a = start + swap_counter;
        let b = ((start + count) - 1) - swap_counter;
        swap(slice, a, b);
    }
}

#[test]
fn test_reverse_sub_slice() {
    let mut slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    reverse_sub_slice(&mut slice, 2, 3);
    assert_eq!([0, 1, 4, 3, 2, 5, 6, 7, 8, 9, 10], slice);

    slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    reverse_sub_slice(&mut slice, 5, 7);
    assert_eq!([5, 1, 2, 3, 4, 0, 10, 9, 8, 7, 6], slice);
}

fn swap(slice: &mut [usize], a: usize, b: usize) {
    let a = a % slice.len();
    let b = b % slice.len();

    let swap = slice[a];
    slice[a] = slice[b];
    slice[b] = swap;
}

#[test]
fn test_swap() {
    let mut slice = [0, 1, 2, 3, 4];
    swap(&mut slice, 0, 4);
    assert_eq!([4, 1, 2, 3, 0], slice);

    slice = [0, 1, 2, 3, 4];
    swap(&mut slice, 1, 5);
    assert_eq!([1, 0, 2, 3, 4], slice);
}