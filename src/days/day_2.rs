use util::files::read_file_to_string;

pub fn main() {
    println!("-- Running Day 2 --");
    let input = read_file_to_string("src/days/input/day_2.txt");

    println!("Min-Max Checksum is {}", min_max_checksum(&input));
    println!("Even Dividers Checksum is {}", even_division_checksum(&input));
}

fn min_max_checksum(input: &str) -> u32 {
    let mut sum = 0u32;

    for line in input.lines() {
        let mut max = u32::min_value();
        let mut min = u32::max_value();

        for word in line.split_whitespace() {
            if let Ok(current_num) = word.parse::<u32>() {
                if current_num > max {
                    max = current_num;
                }

                if current_num < min {
                    min = current_num;
                }
            } else {
                println!("Bad Input");
            }
        }

        sum = sum + (max - min);
    }

    sum
}

fn even_division_checksum(input: &str) -> u32 {
    let mut sum = 0u32;

    for line in input.lines() {
        let nums: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

        for a_index in 0..nums.len() {
            for b_index in (a_index + 1)..nums.len() {
                let a = nums[a_index];
                let b = nums[b_index];

                if a > b && a % b == 0 {
                    sum = sum + (a / b);
                    break;
                } else if b % a == 0 {
                    sum = sum + (b / a);
                    break;
                }
            }
        }
    }

    sum
}