// Practice file: Concurrency Practice

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // Standard Way
    let handle = thread::spawn(|| {
        println!("Hello from Thread!");
    }); // Creates a new thread
    handle.join().unwrap(); // Blocks until thread finished

    // Data can't be passed to thread from outside, so we have use "move"
    let data = vec![1, 2, 3, 4];
    // Without move, when we tries to access "data" inside the thread
    // let h = thread::spawn(|| {
    //     println!("{:?}", data);
    // }); // Throws: closure may outlive the current function, but it borrows `data`, which is owned by the current function
    
    let h = thread::spawn(move || {
        println!("{:?}", data);
    });
    h.join().unwrap();

    // park / unpark
    let parked = thread::spawn(|| {
        thread::park();     // Sleeps until unparked
        println!("Resumed");
    });
    let handle = parked.thread().clone();
    thread::sleep(Duration::from_millis(100)); // Thread parked(), we delayed unpark and
                                                 // until unpark, it won't print "Resumed"
    handle.unpark();

    // Send & Sync: Automatically derived by the compiler based on the types our struct contains
    // Send - Type can be moved to another thread
    // Sync - &T can be shared across the threads
    
    // Message Passing - mpsc channels
    // Unbound & Async
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone(); // multi-producer: clone sender
     
    thread::spawn(move || tx.send(String::from("Hello from tx")).unwrap());
    thread::spawn(move || tx2.send(String::from("Hello from tx1")).unwrap());

     // rx is the sole consumer (single-consumer)
    for msg in rx { // rx implements IntoIterator; blocks until all Senders drop
        println!("{}", msg);
    }

    // Bound & Sync
    let (tx, rx) = mpsc::sync_channel::<u32>(5); // buffer of 5

    // send() BLOCKS when buffer is full — natural back pressure
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap(); // blocks at i=5 until receiver consumes
        }
    });

    for val in rx { println!("{}", val); }
}