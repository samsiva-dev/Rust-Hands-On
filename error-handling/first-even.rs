/*
    Write first_even(nums: &[i32]) -> Option<i32> that returns the first even number 
    or None. Use if let to handle it at the call site.
*/

/*
    Option<T> {
        Some(T), // contains a value of type T
        None, // represents the absence of a value
    }
*/

fn first_even(nums: &[i32]) -> Option<i32> {
    for &num in nums {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 6, 7];

    match first_even(&numbers) {
        Some(even) => println!("First even number: {}", even),
        None => println!("No even number found."),
    }
}