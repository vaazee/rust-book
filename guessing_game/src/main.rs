use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Generating secret no...");

    let secret_no = rand::thread_rng().gen_range(1..=100);
    println!("Generated secret no:{}", secret_no);
    loop {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read_line");

        println!("Your guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue, 
        };

        match guess.cmp(&secret_no) {
            Ordering::Equal => {
                println!("Correct, Victory!");
                break;
            } 
            Ordering::Less => println!("Guess is smaller, try again.."),
            Ordering::Greater => println!("Guess is greater, try again.."),
        }
    }
}
