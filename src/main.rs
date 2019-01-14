extern crate rand;
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = guess.trim().parse().expect("Please enter only numbers!");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won! good job");
                break;
            }
        };
    }
}
