use crate::transaction::{Transaction, Status};
// `crate` refers to the root of the current crate (core/src/lib.rs)
// `crate::transaction` is the transaction module within it

pub fn approve(txn: &mut Transaction) -> Result<(), String> {
    if txn.amount <= 0.0 {
        return Err(String::from("amount must be positive"));
    }
    if txn.amount > 100_000.0 {
        return Err(String::from("amount exceeds limit"));
    }

    txn.status = Status::Approved;
    txn.set_note("approved by validator");  // works — pub(crate) is visible here
    // txn.note = ...                       // ❌ would fail — `note` is private
    Ok(())
}

pub fn reject(txn: &mut Transaction, reason: &str) {
    txn.status = Status::Rejected(reason.to_string());
    txn.set_note(reason);
}