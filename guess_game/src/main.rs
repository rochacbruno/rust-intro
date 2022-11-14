use rand::Rng;
use std::cmp::Ordering;
use std::io; // NEW

fn main() {
    println!("Guess the Number");
    let number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {number}");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error happened");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };
        println!("You guessed {guess}");
        // NEW
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
