use std::io;

fn main() {

    println!("Enter the string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

  
    let input = input.trim().to_lowercase();

    let reversed_input: String = input.chars().rev().collect();

    if input == reversed_input {
        println!("It is a PALINDROME!!!");
    } else {
        println!("It is not a PALINDROME!!!");
    }
}
