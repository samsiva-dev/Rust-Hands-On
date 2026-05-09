/*
    Given a Vec<i32>, return the sum of all even numbers squared.
    Input:  [1, 2, 3, 4, 5, 6]
    Output: 56  // 2²+4²+6² = 4+16+36
    Concepts practiced: .iter(), .filter(), .map(), .sum()
*/

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let sum_of_squares: i32 = nums.iter()
        .filter(|&x| x % 2 == 0) // Filter even numbers
        .map(|x| x * x)          // Square the even numbers
        .sum();                   // Sum them up

    println!("Sum of squares of even numbers: {}", sum_of_squares);
}