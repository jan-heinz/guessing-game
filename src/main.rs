use std::io;
use rand::Rng;
use std::cmp::Ordering;
 
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);
 
    println!("Input your guess");
    let mut guess = String::new();
 
    io::stdin()
        // appends user input to guess string AND returns a Result value
        // Result is an enum, variants are
        //  - Ok (value is stored inside this)
        //  - Err (contains info for why fail occured)
        .read_line(&mut guess)
        .expect("Failed to read line"); // expect is a method that can be called on Result, if Err program will crash
 
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guessed: {}", guess);
 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
} 
 