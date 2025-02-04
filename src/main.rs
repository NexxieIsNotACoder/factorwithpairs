use std::io;
use factor::factor::factor;

fn main() {
    println!("Input number for factors: ");
    // creates a new variable called input, places user input into it
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // convert input to i64 data type as the factor function returns an i64 and i think we'll need that.
    let input: i64 = input.trim().parse().expect("Please type a number!");
    print!("Factors for this number are: ");
    // since the factor function returns an array, we just print everything from the array
    let factors = factor(input);
    for element in factors.iter() {
        print!(" {element}");
    }
    println!();
    println!("Factor pairs: ");
    for element in factors.iter() {
        // creates the pairs by dividing the input number by one of the factors to get... another factor! 
        print!(" ({}, {})", element, input / element);
    }
}
