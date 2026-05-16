use banking::{Account, Ledger};   // import banking types

fn main() {
    let mut ledger = Ledger::new();

    // Create some accounts
    ledger.add_account(Account::new(1, "Alice", 1000.0));
    ledger.add_account(Account::new(2, "Bob", 500.0));

    println!("Initial balances:");
    ledger.print_balances();

    // Perform a transfer
    match ledger.transfer(1, 2, 200.0) {
        Ok(_) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    println!("Final balances:");
    ledger.print_balances();
}