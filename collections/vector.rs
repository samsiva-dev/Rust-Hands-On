fn main() {
    // Approach 1: Creating a vector
    // let nums: Vec<i32> = Vec::new(); // Immutable by default, Read-only
    let mut nums: Vec<i32> = Vec::new(); // Now can perform add/update/delete
    let mut strings: Vec<String> = Vec::new();

    // let mut nums: Vec<i32> = Vec::with_capacity(10); // the vec pre-allocates heap memory
    // println!("len: {}, capacity: {}", nums.len(), nums.capacity());

    // Approach 2: Using vec![]
    // let nums = vec![1, 2, 3, 4, 5];

    nums.push(4);    // Appends to the right
    nums.push(3); 

    strings.push(String::from("hello")); 

    /*
        Here nums is i32 vector, and i32 has Copy trait, so when we assigned directly, 
        the values are Copied. If Vector is a String collection, then it will become
        ownership problem. So we have to use references '&' or .clone()
    */
    // Reading using index
    // let first = nums[0];    // i32 has copy trait, so copied
    // println!("First: {}", first);
    let first = &nums[0];   // reference to the data
    println!("First: {}", first);

    // Reading using get() return Options<&T>
    let second = nums.get(1);
    match second {
        Some(second) => println!("Second: {}", second),
        None => println!("No element exists"),
    };

    // let third = &nums[2]; // Creates panic: index out of bounds: the len is 2 but the index is 2
    let third = nums.get(2); // Gives us a option to handle
    match third {
        Some(third) => println!("Third: {}", third),
        None => println!("There is no third element in nums")
    };

    // Iterating...
    // Uses Move trait, so creates problem in feature.
    // Why i32 has Copy Trait? Because Vec has no Copy trait and below loop is equivalent
    // to nums.to_iter() which takes the ownership of nums and drops it once loop completed
    // for num in nums {
    //     println!("{}", num);
    // }

    // Moves the data, so if we want to using strings after this a panic will be created
    // for s in strings {
    //     println!("{}", s);
    // }
    // println!("{}", strings[0]); // panics:  value borrowed here after move

    // So, ALWAYS to REMEMBER to use '&' as common standards across different type of vectors
    // for s in &strings {
    //     println!("{}", s);
    // }
    // println!("{}", strings[0]); // Now WORKs

    // Modify by iterating
    for num in &mut nums {
        *num *= 2;
    }
    // println!("Doubled nums vector:");
    // for num in &nums {
    //     println!("{}", num);
    // }

    // Vector Helper functions
    println!("Length = {}", nums.len());   // returns a usize
    println!("IsEmpty = {}", nums.is_empty()); // returns a boolean
    println!("Does nums contains 2? {}", nums.contains(&2)); // returns a boolean
    nums.sort();  // sort in-place
    // for num in &nums {
    //     println!("{}", num);
    // }
    nums.dedup(); // removes consecutive duplicates (sort first)
    nums.retain(|x| *x > 7);  // retains the elements which passes the condition (in-place)
    // for num in &nums {
    //     println!("{}", num);
    // }
    nums.extend([9, 10]);  // extends the vec with new elements
    // for num in &nums {
    //     println!("{}", num);
    // }
    nums.truncate(2); // reduces the vector to given length
    // for num in &nums {
    //     println!("{}", num);
    // }
    nums.clear(); // empties the vector
    for num in &nums {
        println!("{}", num);
    }
    println!("IsEmpty = {}", nums.is_empty()); 
}