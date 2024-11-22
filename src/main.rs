use rand::{thread_rng, Rng};
use std::io;

fn main() {
    println!("Welcome to Random Password Generator!");
    println!("Enter the desired password length:");
    let length: usize = get_input().trim().parse().expect("Please type a valid number!");
    let password = generate_password(length);
    println!("Generated Password: {}", password);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn generate_password(length: usize) -> String {
    let character = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
    let mut rng = thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..character.len());
            character.chars().nth(idx).unwrap()
        })
        .collect()
}
