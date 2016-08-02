extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("input guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN");
                continue;
            },
        };

        println!("input: {}", guess);

        let paired: (bool, u32) = (false, guess);
        match paired {
            (true, 20...26) => println!("t 20->26"),
            (true, _) => println!("t !20-26"),
            (_, 40...49) => println!("x 40->49"),
            _ => println!("not true"),
        }

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("higher"),
            Ordering::Greater => println!("lower"),
            Ordering::Equal   => {
                println!("match");
                break;
            },
        }
    }
}
