/*
    Key elements of Asyn Programming in Rust are 
    "Futures, async and await"

    future - a value that may not be ready now but will become ready at
    some point in the future.
     - In rust, futures are types that implement the "Future" trait. Each future
       holds its own information about the progress that has been made and what 
       "ready" means.

    async - we can apply "async" keyword to blocks and functions to specify that
    they can be interrupted and resumed.
     - Within an async block or aysnc function, we can use "await" keyword to await 
     a future. Any point where you await a future within an async block or function 
     is a potential spot for that block or function to pause and resume. The process 
     of checking with a future to see if its value is available yet is called polling.
*/

// Futures
// Every async operation in Rust produces a Future - a value representing a computation
// that will complete later
// trait Future {
//     type Output;
//     fn poll(&mut self, cx: &mut Context) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T), // computation complete, here is the result
//     Pending,  // not done yet, will notify when ready
// }

async fn fetch_data() -> String {
    String::from("data...")
}

// the above is equivalent to
// fn fetch_data() -> impl Future<Output = String> {
//     async { String::from("data...") }
// }

async fn process() {
    let data = fetch_data().await;  // suspend here until fetch_data completes
    println!("{}", data);
}

// The key rule: .await can only be used inside async fn or async blocks. 
// You cannot call .await in a regular function — there is no executor to suspend into.