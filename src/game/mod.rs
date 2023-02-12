#![allow(unused)]

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn start() {
  println!("Guess the number!");
    
  let secret_number = rand::thread_rng().gen_range(1..=100);
  loop {
    println!("Please enter your guess");
    let mut guess: String = String::new();
  
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");
  
    // We trim & parse user input to type u32 and check for error
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
  
    println!("You guessed: {}", guess);
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("Too you win!");
        break;
      },
    }
  }
}
