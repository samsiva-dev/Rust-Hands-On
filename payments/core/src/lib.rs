pub mod transaction;     // load src/transaction.rs → becomes core::transaction
pub mod validation;      // load src/validation.rs  → becomes core::validation

// re-export the most commonly used types at the top level
// callers write `core::Transaction` instead of `core::transaction::Transaction`
pub use transaction::Transaction;
pub use transaction::Status;