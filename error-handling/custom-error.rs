/*
    Define an AppError enum with variants:
    1. Io(std::io::Error)
    2. Parse(std::num::ParseIntError)
    3. NotFound(String)
    Implement std::fmt::Display and std::error::Error for it.
    Then rewrite config-reader.rs function returning Result<u16, AppError>.
*/

use std::fs;
use std::num::ParseIntError;
use std::fmt;
use std::error::Error;

// Define the AppError enum (custom error type)
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
    NotFound(String),
}

// Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

// Implement the Error trait for AppError
impl Error for AppError {}

// Rewrite the read_port function to return Result<u16, AppError>
fn read_port(path: &str) -> Result<u16, AppError> {
    // Convert io::Error to AppError using map_err
    let content = fs::read_to_string(path).map_err(AppError::Io)?;

    // Check if the content is empty and return a NotFound error if it is
    if content.trim().is_empty() {
        return Err(AppError::NotFound(format!("No port value found in '{}'", path)));
    }
    
    // Convert ParseIntError to AppError using map_err
    let port = content.trim().parse::<u16>().map_err(AppError::Parse)?;
    Ok(port)
}

fn main() {
    match read_port("port-1.txt") {
        Ok(port) => println!("Port number: {}", port),
        Err(e) => println!("Error: {}", e),
    }
}