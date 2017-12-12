use util::files::read_file_to_string;

pub fn main() {
    println!("-- Running Day 4 --");
    println!("{}", read_file_to_string("src/days/input/day_4.txt").lines().count());
}