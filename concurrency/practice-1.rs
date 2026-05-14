use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0_u32));

    let handles: Vec<_> = (0..8).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || {
            let mut guard = c.lock().unwrap(); // blocks; returns MutexGuard<u32>
            *guard += 1;
            // guard dropped here → lock released
        })
    }).collect();

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", *counter.lock().unwrap()); // 8
}