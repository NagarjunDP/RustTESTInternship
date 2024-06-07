use std::io;

fn main() {
    //Input from user
    println!("Enter the string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline character and convert to lowercase
    let input = input.trim().to_lowercase();

    let reversed_input: String = input.chars().rev().collect();
    //Checking if input is equal to reversed string
    if input == reversed_input {
        println!("It is a PALINDROME!!!");
    } else {
        println!("It is not a PALINDROME!!!");
    }
}
