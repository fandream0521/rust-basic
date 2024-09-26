use std::{cmp::Ordering, io};

use rand::Rng;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let random_num = rand::thread_rng().gen_range(1..101);

    loop {
        let Some(guess) = read_number() else {
            break;
        };
        match random_num.cmp(&guess) {
            Ordering::Less => println!("Too big!"),
            Ordering::Greater => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn read_number() -> Option<u32> {
    loop {
        let mut guess = String::new();
        if let Ok(_) = io::stdin().read_line(&mut guess) {
            if let Ok(num) = guess.trim().parse::<u32>() {
                return Some(num);
            } else {
                if "quit".eq(&guess.trim().to_lowercase()) {
                    return None;
                } else {
                    println!("Please input a number!");
                }
            }
        }
    }
}
