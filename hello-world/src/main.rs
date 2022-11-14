use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number");

    let number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {number}");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // unwrap the value
            Err(_) => {
                println!("Invalid Number");
                continue; // restart the loop
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations!");
                break;
            }
        }
    }
}
