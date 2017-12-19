mod days;
mod util;

use days::DAYS;

use std::io;
use std::env;

fn main() {
    let day_fns = DAYS;

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