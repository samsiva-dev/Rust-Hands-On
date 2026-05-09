/*
    Implement a struct Fibonacci that implements the Iterator trait, yielding Fibonacci numbers.
    rustlet fibs: Vec<u64> = Fibonacci::new().take(8).collect();
    // Expected: [0, 1, 1, 2, 3, 5, 8, 13]
    Concepts practiced: impl Iterator for Struct, type Item, fn next()
*/

struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let current = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(current)
    }
}

fn main() {
    let fibs: Vec<u64> = Fibonacci::new().take(8).collect();
    println!("{:?}", fibs); // Expected: [0, 1, 1, 2, 3, 5, 8, 13]
}