#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    
    io::stdin().read_line(&mut name).expect("Didn't receive input");
    
    println!("Hello {}! {}", name, greeting);
}