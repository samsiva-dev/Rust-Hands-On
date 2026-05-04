/*
    Write a function process_user_input(raw: &str) -> Result<String, AppError> that:
    1. Trims and validates the string is non-empty (return NotFound if empty)
    2. Parses it as i32
    3. Doubles the value
    4. Returns "Result: <value>" as a String

    Chain operations using ?, map, and and_then — avoid nested match blocks.
*/

use std::num::ParseIntError;
use std::fmt;
use std::error::Error;

// Define the AppError enum (custom error type)
#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    NotFound(String),
}

// Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

// Implement the Error trait for AppError
impl Error for AppError {}  

fn process_user_input(raw: &str) -> Result<String, AppError> {
    raw.trim() // Step 1: Trim the input
        .parse::<i32>() // Step 2: Parse as i32
        .map_err(AppError::Parse) // Convert ParseIntError to AppError
        .and_then(|num| { // Step 3: Double the value
            if num == 0 {
                Err(AppError::NotFound("Input cannot be zero".to_string()))
            } else {
                Ok(num * 2)
            }
        })
        .map(|doubled| format!("Result: {}", doubled)) // Step 4: Format the result
}

fn main() {
    match process_user_input("  21  ") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }

    match process_user_input("   ") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }

    match process_user_input("0") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}