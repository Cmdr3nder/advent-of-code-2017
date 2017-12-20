use util::files::read_file_to_string;

const STRAND_SIZE: usize = 256;

pub fn main() {
    println!("-- Running Day 10 --");
    let input = &read_file_to_string("src/days/input/day_10.txt");
    let lengths = parse_lengths(input);
    let final_strand = execute_lengths(lengths);
    println!("Multiple of first couple is {} = {} * {}", final_strand[0] * final_strand[1], final_strand[0], final_strand[1]);
}

fn parse_lengths(input: &str) -> Vec<usize> {
    input.split(',').map(|s| {
        s.trim().parse::<usize>().unwrap()
    }).collect()
}

fn execute_lengths(lengths: Vec<usize>) -> [u32; STRAND_SIZE] {
    let mut strand = initialize_strand();
    let mut current_index = 0;
    let mut skip_size = 0;

    for length in lengths {
        //Rearrange
        let reverse_values: Vec<u32> = generate_reversed_indices(current_index, length).into_iter().map(|index| strand[index]).collect();
        for index in 0..reverse_values.len() {
            strand[(index + current_index) % STRAND_SIZE] = reverse_values[index];
        }

        //Move
        current_index = (current_index + length + skip_size) % STRAND_SIZE;
        skip_size = skip_size + 1;
    }

    strand
}

fn initialize_strand() -> [u32; STRAND_SIZE] {
    let mut strand = [0u32; STRAND_SIZE];

    for index in 0..STRAND_SIZE {
        strand[index] = index as u32;
    }

    strand
}

fn generate_reversed_indices(current_index: usize, length: usize) -> Vec<usize> {
    let mut indices = Vec::new();

    for index in current_index..(current_index + length) {
        indices.push(index % STRAND_SIZE);
    }

    indices.into_iter().rev().collect()
}