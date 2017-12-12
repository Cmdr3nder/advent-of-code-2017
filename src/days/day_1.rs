use util::files::read_file_to_string;

pub fn main() {
    println!("-- Running Day 1 --");
    let input = read_file_to_string("src/days/input/day_1.txt");

    let checksum = next_char_checksum(&input);
    println!("Next Char based Checksum is {}", checksum);

    let checksum = half_around_checksum(&input);
    println!("Half Around based Checksum is {}", checksum);
}

fn next_char_checksum(input: &str) -> u32 {
    let mut char_iter = input.chars();
    let first_char = char_iter.next().unwrap();
    let mut current_char = first_char;
    let mut sum = 0u32;

    for next_char in char_iter {
        if current_char == next_char {
            match current_char.to_digit(10) {
                Some(ref n) => sum = sum + n,
                None => println!("Bad Input")
            };
        }

        current_char = next_char;
    }

    if current_char == first_char {
        match current_char.to_digit(10) {
            Some(ref n) => sum = sum + n,
            None => println!("Bad Input")
        }
    }

    sum
}

fn half_around_checksum(input: &str) -> u32 {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let jump = digits.len() / 2;
    let mut sum = 0u32;

    for idx in 0..digits.len() {
        let digit = digits[idx];
        let jump_digit = digits[(idx + jump) % digits.len()];

        if digit == jump_digit {
            sum = sum + digit;
        }
    }

    sum
}