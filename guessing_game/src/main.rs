use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let random_int = rand::thread_rng().gen::<u32>();

    println!(
        "From [1..=100] = {} and int[1..=4.3bil] = {}",
        secret_number, random_int
    );

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

        match secret_number.cmp(&guess) {
            Ordering::Less => println!("Too high!"),
            Ordering::Greater => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
