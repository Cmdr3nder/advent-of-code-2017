use std::collections::HashMap;

pub fn main() {
    println!("-- Running Day 6 --");
    let input: [u16; 16] = [14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];
    let (cycles, cycles_in_loop) = cycles_until_dupe(input);
    println!("Cycles before duplication, {}", cycles);
    println!("Cycles in the loop, {}", cycles_in_loop);
}

fn cycles_until_dupe(input: [u16; 16]) -> (usize, usize) {
    let mut states = HashMap::new();
    let mut next_array = input;
    let mut count = 1usize;
    states.insert(next_array, count);

    while { //Crazy do-while...
        next_array = reballance_array(next_array);
        if states.contains_key(&next_array) {
            false
        } else {
            states.insert(next_array, count);
            count = count + 1;
            true
        }
    } {} ;

    let cycles_in_loop = count - states.get(&next_array).unwrap();

    (count, cycles_in_loop)
}

fn reballance_array(arr: [u16; 16]) -> [u16; 16] {
    let mut most_index = 0;
    let mut result: [u16; 16] = arr;

    for i in 0..result.len() {
        if result[i] > result[most_index] {
            most_index = i;
        }
    }

    let mut distribute = result[most_index];
    result[most_index] = 0;

    let mut distribute_index = (most_index + 1) % result.len();

    while distribute > 0 {
        result[distribute_index] = result[distribute_index] + 1;
        distribute = distribute - 1;
        distribute_index = (distribute_index + 1) % result.len();
    }

    result
}