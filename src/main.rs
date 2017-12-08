mod days;
mod util;

use days::day_1::main as day_1;
use days::day_2::main as day_2;
use days::day_3::main as day_3;
use util::cardinal::Cardinal;

fn main() {
    day_1();
    day_2();
    day_3();

    let card = Cardinal::East;
    println!("Cardinal {}", card.turn_left());
}
