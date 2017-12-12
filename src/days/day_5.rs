use util::files::read_file_to_string;

pub fn main() {
    println!("-- Running Day 5 --");
    let input = &read_file_to_string("src/days/input/day_5.txt");

    println!("It took {} steps to jump out.", count_steps_to_jump_out(input, calc_jump_as_simple_increment));
    println!("It took {} steps to jump out, extra logic.", count_steps_to_jump_out(input, calc_jump_with_logic));
}

fn count_steps_to_jump_out(input: &str, calc_jump: fn(isize) -> isize) -> u32 {
    let mut count = 0u32;
    let mut index = 0isize;

    let mut instructions: Vec<isize> = input.lines().map(|l| l.trim().parse::<isize>().unwrap()).collect();

    while index >= 0 && index < instructions.len() as isize {
        let jump = instructions[index as usize];
        instructions[index as usize] = calc_jump(jump);
        index = index + jump;
        count = count + 1;
    }

    count
}

fn calc_jump_as_simple_increment(jump: isize) -> isize {
    jump + 1
}

fn calc_jump_with_logic(jump: isize) -> isize {
    if jump >= 3 {
        jump - 1
    } else {
        jump + 1
    }
}