/*
By: <Aksharan Sudarshan>
Date: 2026-11-05
Program Details: <Program Description Here>
*/
use rand::Rng;
use std::io;

fn main() {
       let mut rng = rand::rng();
    println!("Hello guessing_game!");
     let random_int: i32 = rng.random_range(1..=100);
    println!("Random integer between 1 and 100: {}", random_int);
    let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");

let number: i32 = input.trim().parse().expect("Please type a number!");
println!("You typed: {}", number);

loop{






}

}

