/*
    Write a function read_port(path: &str) -> Result<u16, String> that:
    1. Reads a file
    2. Parses the content as a u16 port number
    3. Propagates errors using ?

    You'll hit a compile error because fs::read_to_string returns io::Error and parse() 
    returns ParseIntError — both different types. Figure out how to make ? work for both.
*/

use std::fs;
use std::num::ParseIntError;

// Old version without error conversion
// fn read_port(path: &str) -> Result<u16, String> {
//     let content = fs::read_to_string(path)?; // Returns io::Error on failure
//     let port: u16 = content.trim().parse()?; // Returns ParseIntError on failure

//     // Parsed successfully, return the port number
//     Ok(port)
// }

// New version with error conversion
fn read_port(path: &str) -> Result<u16, String> {
    // Convert io::Error to String using map_err
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

    // Convert ParseIntError to String using map_err
    let port: u16 = content.trim().parse().map_err(|e: ParseIntError| e.to_string())?;

    Ok(port)
}


fn main() {
    match read_port("port.txt") {
        Ok(port) => println!("Port number: {}", port),
        Err(e) => println!("Error: {}", e),
    }
}