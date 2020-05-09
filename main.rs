use std::io;
use std::process;

fn main() {

    // Create new variable for input
    let mut input_number = String::new();

    // Create variables for fibonacci sequence
    let mut x: u32 = 0;
    let mut y: u32 = 1;
    let mut z: u32 = 0;

    println!("Enter number: ");

    // Read from user
    io::stdin().read_line(&mut input_number).expect("Failed to read line!");

    // Turn string to int
    // unwrap_or() will set variable to 1, if integer cannot be parse
    let mut input_number: u32 = input_number.trim().parse().unwrap_or(1);

    // Loop until nth number is reached
    for _ in 1..=input_number
    {
        println!("{}", x);
        z = x + y;
        x = y;
        y = z;
    }
}
