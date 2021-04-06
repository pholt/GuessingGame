use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I'm thinking of a number between 1 and 100! You have 20 guesses...");

    let mut tries: u32 = 0;

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                tries = tries + 1;
                num
            },
            Err(_) => {
                println!("Input must be a number!");
                continue
            },
        };

        if tries >= 20 {
            println!("You've guessed 20 times? You lose! The number was {}, by the way.", secret_number);
            break;
        } else {
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("The number was: {}! It took you {} guesses!", secret_number, tries);
                    break;
                }
            }
        }
    }
}
