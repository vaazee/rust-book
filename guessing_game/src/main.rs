use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Generating secret no...");

    let secret_no = rand::thread_rng().gen_range(1..=100);
    println!("Generated secret no:{}", secret_no);

    println!("Enter your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read_line");

    println!("Your guess: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Invalid input, enter a valid number.");

    match guess.cmp(&secret_no) {
        Ordering::Equal => println!("Correct, Victory!"),
        Ordering::Less => println!("Guess is smaller, try again.."),
        Ordering::Greater => println!("Guess is greater, try again.."),
    }
}
