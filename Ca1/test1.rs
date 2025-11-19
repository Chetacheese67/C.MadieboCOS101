// Rust program to convert temperature in celsius to farenheit
 use std::io;


 fn main() {

 	let mut input1 = String::new();

 	println!("Enter temperature in celsius");
 	io::stdin().readline(&mut input1).expect("Not a valid temperature in range");
 	let celcius:f32 = input1.trim().parse().expect("Not a valid number");



 	let farenheit :f32 = (a * 9.0)/5.0 + 32.0
 	let mut kelvin:f32 = (a + 273.15);

println!("the temperature in farenheit is {}",farenheit);
println!("the temperature ib kelvin is{}",kelvin);

if celcius >= 0{
println!("its is below freezing point");
}
if celcius >=30 {
println!("it is at normal range");
}
if celcius <=30{
println!("It is hot");
}

}
