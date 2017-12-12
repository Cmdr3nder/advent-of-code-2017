mod days;
mod util;

use days::day_1::main as day_1;
use days::day_2::main as day_2;
use days::day_3::main as day_3;
use days::day_4::main as day_4;
use days::day_5::main as day_5;
use days::day_6::main as day_6;

use std::io;
use std::env;

fn main() {
    let day_fns = [day_1, day_2, day_3, day_4, day_5, day_6];

    if env::args().len() > 1 {
        day_fns[day_fns.len() - 1]();
        return;
    }

    loop {
        println!("Which day do you want to run? (1 - {})", day_fns.len());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");

        if choice.trim().len() == 0 {
            day_fns[day_fns.len() - 1]();
        } else {
            match choice.trim().parse::<usize>() {
                Ok(c) => {
                    if c > 0 && c <= day_fns.len() {
                        day_fns[c - 1]();
                    } else {
                        println!("Woops invalid input, Out of Range");
                    }
                },
                Err(e) => println!("Woops invalid input, {}", e)
            }
        }

        println!("");
    }
}