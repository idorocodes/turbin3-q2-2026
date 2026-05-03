use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(0..100);

    let mut stack = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

       
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                stack += 1;

                println!("Too small!");
            }
            Ordering::Greater => {
                stack += 1;

                println!("Too big!")
            }
            Ordering::Equal => {
                println!("You win!, the lucky number was {}", secret_number);
                break;
            }
        }
        if stack == 10 {
            println!("Guess finished, number is {}", secret_number);
            break;
        }
    }
}
