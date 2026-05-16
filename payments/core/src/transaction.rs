// this entire file is the `transaction` module
// because lib.rs declared `pub mod transaction;`

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id:     u64,
    pub amount: f64,
    pub status: Status,
    note:       String,    // private — only this module can access it
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pending,
    Approved,
    Rejected(String),      // carries a reason
}

impl Transaction {
    // public constructor — the only way to create a Transaction from outside
    pub fn new(id: u64, amount: f64) -> Self {
        Transaction {
            id,
            amount,
            status: Status::Pending,
            note:   String::new(),   // callers can't set this directly
        }
    }

    // public method — readable outside
    pub fn is_approved(&self) -> bool {
        self.status == Status::Approved
    }

    // pub(crate) — visible inside the `core` crate, not outside
    pub(crate) fn set_note(&mut self, note: &str) {
        self.note = note.to_string();
    }

    // private — only code in this file can call this
    fn internal_log(&self) {
        println!("[log] txn {} note: {}", self.id, self.note);
    }
}