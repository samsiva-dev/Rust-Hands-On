use core::{Transaction, Status};    // both re-exported from core's lib.rs
use core::validation;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // cargo run --bin cli -- 42 500.0
    let id:     u64 = args[1].parse().unwrap_or(0);
    let amount: f64 = args[2].parse().unwrap_or(0.0);

    let mut txn = Transaction::new(id, amount);

    match validation::approve(&mut txn) {
        Ok(())   => println!("txn {} approved for ${}", txn.id, txn.amount),
        Err(msg) => println!("txn {} rejected: {}", txn.id, msg),
    }
}