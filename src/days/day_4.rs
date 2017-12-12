use util::files::read_file_to_string;

use std::collections::HashSet;

pub fn main() {
    println!("-- Running Day 4 --");
    let input = &read_file_to_string("src/days/input/day_4.txt");

    println!("Valid passphrase count = {}", no_dupe_words_count(input));
    println!("Valid passphrase anagram count = {}", no_anagram_dupes_count(input));
}

fn no_dupe_words_count(input: &str) -> u32 {
    let mut count = 0u32;

    for line in input.lines() {
        let mut words = HashSet::new();
        let mut no_dupes = true;

        for word in line.split_whitespace() {
            if words.contains(word) {
                no_dupes = false;
                break;
            } else {
                words.insert(word);
            }
        }

        if no_dupes {
            count = count + 1;
        }
    }

    count
}

fn no_anagram_dupes_count(input: &str) -> u32 {
    let mut count = 0u32;

    for line in input.lines() {
        let mut words = HashSet::new();
        let mut no_dupes = true;

        for word in line.split_whitespace().map(de_anagram) {
            if words.contains(&word) {
                no_dupes = false;
                break;
            } else {
                words.insert(word);
            }
        }

        if no_dupes {
            count = count + 1;
        }
    }

    count
}

fn de_anagram(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}