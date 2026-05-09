/*
Given two Vec<i32>, produce a single sorted Vec<i32> of unique elements present in both vectors (intersection).
Input:  a = [1, 2, 3, 4, 5],  b = [3, 4, 5, 6, 7]
Output: [3, 4, 5]
Concepts practiced: HashSet, .iter(), .filter(), .cloned(), .collect(), sort()
*/

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![3, 4, 5, 6, 7];

    // Create a HashSet from vector 'a' for O(1) lookups
    let set_a: std::collections::HashSet<_> = a.iter().cloned().collect();

    // Filter elements in 'b' that are also in 'set_a', then collect into a vector
    let mut intersection: Vec<_> = b.iter()
        .filter(|&x| set_a.contains(x)) // Keep only elements that are in set_a
        .cloned() // Clone the values to get owned i32s instead of references
        .collect();

    // Sort the resulting vector of unique elements
    intersection.sort();

    println!("Intersection of a and b: {:?}", intersection);
}