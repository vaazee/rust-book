use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input the number.");
    let mut guess = String::new();
    io.stdin()
    .read_line(&mut guess)
    .expect("Failed to read input.");
    println!("You entered {}", guess);
}
