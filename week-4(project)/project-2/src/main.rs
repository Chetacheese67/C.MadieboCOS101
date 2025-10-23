// Rust program to determine the annual incentive of an employee
use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    println!("Is the employee experienced? (yes or no):");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience = experience_input.trim().to_lowercase();

    println!("Enter age of employee:");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: i32 = age_input.trim().parse().expect("Not a valid number");

    if experience == "yes" {
        if age >= 40 {
            println!("The annual incentive is ₦1,560,000");
        } else if age >= 30 && age < 40 {
            println!("The annual incentive is ₦1,480,000");
        } else if age < 28 {
            println!("The annual incentive is ₦1,300,000 per month");
        } else {
            println!("The annual incentive is ₦1,300,000");
        }
    } else {
        println!("The annual incentive is ₦100,000");
    }
}
