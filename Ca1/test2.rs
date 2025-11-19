// payroll calculator

use std::io;

fn main() {
	
println!("Payroll Calculation");

let mut input2 = Sstring::new();

	
// input name
	println!("Please enter your name");
    let &mut name = String::new();
	io::stdin()
	.read_line(&mut name)
	.expect("Failed to read input")
	println!("Your name is: {}", name);

// hours of work input

println!("Eneter the hours of work");
io:stdin().read_line(&mut.input2).expect("Not a valid string");
let hours:f32 = input2.trim().parse().expect("Not a valid number");






if hours <= 40 {
let salary:f32 = (hours * 3000);
}

if hours > 40 {
let salary = (hours * 3000 + 4500); 
}

if salary > 100000 {
let salary = (salary - 2000)

println!("your salary is: {}",salary );