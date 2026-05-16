use core::Transaction;              // works — pub use in lib.rs re-exported it
use core::transaction::Status;      // also works — pub mod transaction is public
use core::validation;               // access the validation module

fn handle_request(id: u64, amount: f64) {
    let mut txn = Transaction::new(id, amount);
    println!("Created: {:?}", txn);

    match validation::approve(&mut txn) {
        Ok(())   => println!("Approved: {:?}", txn),
        Err(msg) => {
            validation::reject(&mut txn, &msg);
            println!("Rejected: {:?}", txn);
        }
    }

    // txn.note = "hack";    // ❌ compile error — note is private to core::transaction
    // txn.set_note("x");    // ❌ compile error — pub(crate) not visible outside core
}

fn main() {
    handle_request(1, 250.0);
    handle_request(2, 999_999.0);   // will be rejected — over limit
    handle_request(3, -50.0);       // will be rejected — negative
}