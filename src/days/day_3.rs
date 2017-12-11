use std::collections::HashMap;
use util::cardinal::Cardinal;

pub fn main() {
    println!("-- Running Day 3 --");
    let input = 347991u32;
    let (x, y) = determine_coords(input as f32);
    let taxicab_distance = (0 - x).abs() + (0 - y).abs();
    println!("Taxicab distance is {}", taxicab_distance);
    println!("Spiral Larger is {}", larger_than_input_spiral(input));
}

fn determine_coords(position: f32) -> (i32, i32) {
    let n = position as f32;
    let k = ((n.sqrt() - 1f32) / 2f32).ceil();
    let t = (2f32 * k) + 1f32;
    let mut m = t.powi(2);
    let t = t - 1f32;

    if n > (m - t) {
        return ((k - (m - n)).ceil() as i32, (-k).ceil() as i32);
    } else {
        m = m - t;
    }

    if n > (m - t) {
        return ((-k).ceil() as i32, (-k + (m - n)).ceil() as i32);
    } else {
        m = m - t;
    }

    if n > (m - t) {
        return ((-k +  (m - n)).ceil() as i32, k.ceil() as i32);
    } else {
        return (k.ceil() as i32, (k - (m - n - t)).ceil() as i32);
    }
}

fn larger_than_input_spiral(input: u32) -> u32 {
    //Prime the "map"
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    map.insert((0i32, 0i32), 1u32);
    //Prime the "pointer"
    let mut location = (0i32, 0i32);
    let mut direction = Cardinal::East;

    while *map.get(&location).unwrap() <= input {
        //Move
        let offset = direction.calc_offset();
        location = (location.0 + offset.0, location.1 + offset.1);
        //Calculate Local Sum
        let local_sum = calc_local_sum(&map, location);
        map.insert(location, local_sum);
        //Figure out next direction to move
        direction = calc_next_direction(&map, location, direction);
    }

    *map.get(&location).unwrap()
}

fn calc_local_sum(map: &HashMap<(i32, i32), u32>, location: (i32, i32)) -> u32 {
    let mut sum = 0u32;

    for x_offset in -1i32..2i32 { //In the future will probably be able to utilize the inclusive range syntax (ex: ..= or ...) when language feature becomes stable.
        for y_offset in -1i32..2i32 {
            let offset_location = (location.0 + x_offset, location.1 + y_offset);

            if let Some(n) = map.get(&offset_location) {
                sum = sum + n;
            }
        }
    }

    sum
}

fn calc_next_direction(map: &HashMap<(i32, i32), u32>, location: (i32, i32), direction: Cardinal) -> Cardinal {
    let my_left = direction.turn_left();
    let offset = my_left.calc_offset();
    let left_location = (location.0 + offset.0, location.1 + offset.1);

    match map.get(&left_location) {
        Some(_) => direction,
        None => my_left
    }
}