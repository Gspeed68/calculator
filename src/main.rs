//! A simple command-line calculator application
//! 
//! This program provides a command-line interface for performing basic arithmetic operations.
//! It runs in a continuous loop until the user chooses to exit, allowing for multiple calculations
//! in a single session.
//! 
//! # Features
//! - Input validation for numbers
//! - Support for basic arithmetic operations (+, -, *, /)
//! - Interactive command-line interface
//! - Continuous operation until user chooses to exit
//! - Clear error messages for invalid inputs
//! 
//! # Usage
//! 1. Run the program with `cargo run`
//! 2. Enter the first number when prompted
//! 3. Enter the second number when prompted
//! 4. Enter the operation (+, -, *, /)
//! 5. View the result
//! 6. Choose whether to continue or exit
//! 
//! # Examples
//! ```
//! Please enter the first number: 
//! 5
//! Please enter the second number: 
//! 3
//! Please enter the operation (+, -, *, /): 
//! +
//! The result is: 8
//! Do you want to continue? (y/n)
//! ```

use std::io;

/// The main entry point of the program
/// 
/// This function implements the main program loop that:
/// 1. Creates a string buffer to store user input
/// 2. Enters an infinite loop that:
///    - Calls the calculator function to perform a calculation
///    - Asks if the user wants to continue
///    - Breaks the loop if the user enters 'n'
/// 
/// # Flow
/// 1. Initialize a string buffer for user input
/// 2. Enter the main program loop
/// 3. Perform a calculation
/// 4. Ask if the user wants to continue
/// 5. Process the user's choice
/// 6. Clear the input buffer for the next iteration
/// 7. Repeat until the user chooses to exit
fn main() {
    // Buffer to store user's choice to continue or exit
    let mut choice = String::new();
    
    // Main program loop
    loop {
        // Perform a calculation
        calculator();
        
        // Ask if user wants to continue
        println!("Do you want to continue? (y/n)");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Exit if user enters 'n'
        if choice.trim() == "n" {
            break;
        }
        
        // Clear the choice buffer for the next iteration
        choice.clear();
    }
}

/// Performs a single calculation operation
/// 
/// This function handles the core calculator functionality:
/// 1. Prompts the user for two numbers
/// 2. Converts the input strings to floating-point numbers
/// 3. Prompts for an arithmetic operation
/// 4. Performs the calculation and displays the result
/// 
/// # Input Process
/// 1. Read the first number from stdin
/// 2. Read the second number from stdin
/// 3. Read the operation from stdin
/// 
/// # Validation
/// - Validates that both inputs are valid numbers
/// - Validates that the operation is one of: +, -, *, /
/// - Handles division by zero with an error message
/// 
/// # Panics
/// This function will panic if:
/// - The user enters non-numeric input for the numbers
/// - The input cannot be read from stdin
/// 
/// # Examples
/// ```
/// Please enter the first number: 
/// 10
/// Please enter the second number: 
/// 5
/// Please enter the operation (+, -, *, /): 
/// *
/// The result is: 50
/// ```
fn calculator() {
    // First number input
    println!("Please enter the first number: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    // Second number input
    println!("Please enter the second number: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    // Convert string inputs to floating-point numbers
    // trim() removes whitespace and newlines
    // parse() converts the string to a number
    // expect() handles any parsing errors
    let num1: f32 = num1.trim().parse().expect("Please type a number!");
    let num2: f32 = num2.trim().parse().expect("Please type a number!");    

    // Operation input
    println!("Please enter the operation (+, -, *, /): ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    // Perform the requested operation
    match operation.trim() {
        "+" => {
            let result = num1 + num2;
            println!("The result is: {}", result);
        },
        "-" => {
            let result = num1 - num2;
            println!("The result is: {}", result);
        },
        "*" => {
            let result = num1 * num2;
            println!("The result is: {}", result);
        },
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero");
            } else {
                let result = num1 / num2;
                println!("The result is: {}", result);
            }
        },
        _ => println!("Invalid operation. Please use +, -, *, or /"),
    }
}
//We need to read two numbers from the user.
//we need to read the operation - additon, subtraction, multiplication, division
//we need to perform the operation on the two numbers
//we need to display the result to the user
