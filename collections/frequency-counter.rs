/*
    Given a Vec<String> of words, return a HashMap<String, usize> with word frequencies.
    Input:  ["apple", "banana", "apple", "cherry", "banana", "apple"]
    Output: {"apple": 3, "banana": 2, "cherry": 1}
    Concepts practiced: HashMap, .entry().or_insert(), iterator over owned values
*/

fn main() {
    let words = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("apple"),
        String::from("cherry"),
        String::from("banana"),
        String::from("apple"),
    ];

    let mut frequency: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for word in words {
        *frequency.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", frequency);
}