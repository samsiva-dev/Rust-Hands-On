use crate::account::Account;

#[derive(Debug, Clone)]
pub struct Ledger {
    accounts: Vec<Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { accounts: Vec::new() }
    }

    pub fn add_account(&mut self, account: Account) {
        println!("Added account {} for {}", account.id, account.owner);
        self.accounts.push(account);
    }

    pub fn get_account(&self, id: u64) -> Option<&Account> {
        self.accounts.iter().find(|acc| acc.id == id)
    }

    pub fn transfer(&mut self, from_id: u64, to_id: u64, amount: f64) -> Result<(), String> {
        let from_idx = self.accounts.iter().position(|a| a.id == from_id)
            .ok_or_else(|| format!("from account {} not found", from_id))?;
        let to_idx = self.accounts.iter().position(|a| a.id == to_id)
            .ok_or_else(|| format!("to account {} not found", to_id))?;

        // withdraw first — check funds before touching destination
        self.accounts[from_idx].withdraw(amount)?;
        self.accounts[to_idx].deposit(amount);
        Ok(())
    }

    pub fn print_balances(&self) {
        for acc in &self.accounts {
            println!("Account {} ({}): balance {}", acc.id, acc.owner, acc.balance());
        }
    }
}