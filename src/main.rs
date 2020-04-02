use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line");
    // If we didn’t use std::io, we could have written 
    // this line as std::io::stdin().
    println!("You guessed: {}", guess);
}
