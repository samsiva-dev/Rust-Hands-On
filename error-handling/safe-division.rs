/*
    Write a function safe_div(a: i32, b: i32) -> Result<i32, String> that returns 
    an error if dividing by zero. Call it and handle both cases explicitly with match.
*/

/*
    Result<T, E> {
        Ok(T), // success case, contains the value of type T
        Err(E), // error case, contains the error of type E
    }
*/

fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match safe_div(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("{}", e),
    }

    match safe_div(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("{}", e),
    }
}