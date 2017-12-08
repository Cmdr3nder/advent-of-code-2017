pub fn main() {
    println!("-- Running Day 3 --");
    let input = 347991i32;
    let (x, y) = determine_coords(input as f32);
    let taxicab_distance = (0 - x).abs() + (0 - y).abs();
    println!("Taxicab distance is {}", taxicab_distance);
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