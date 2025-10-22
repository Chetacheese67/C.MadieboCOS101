use std::io;

fn main() {
    let mut name = String::new();
    let mut age_input = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter your age: ");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");

    let age: i32 = age_input.trim().parse().expect("Please enter a valid number");

    if age >= 18 {
        println!("Welcome to the party, {}!", name.trim());
    } else {
        println!("Oops, you are not of age to enter the party, {}!", name.trim());
    }
}
