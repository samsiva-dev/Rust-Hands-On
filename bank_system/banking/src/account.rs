
#[derive(Debug, Clone)]
pub struct Account {
    pub id:    u64,
    pub owner: String,
    balance:   f64,    // private field — only this module can access it
}

impl Account {
    pub fn new(id: u64, owner: &str, initial: f64) -> Self {
        Account { id, owner: owner.to_string(), balance: initial }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub(crate) fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub(crate) fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(format!("Insufficient funds: balance {}, withdraw {}", self.balance, amount))
        }
    }
}