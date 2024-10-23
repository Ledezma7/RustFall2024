#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }

    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(50.0);
        account.deposit(25.0);
        assert!((account.balance() - 75.0).abs() < 1e-10);

        //if depositing a negative amount
        account.deposit(-10.0);
        assert!((account.balance() - 75.0).abs() < 1e-10);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(30.0);
        assert!((account.balance() - 70.0).abs() < 1e-10);

        // Withdraw a negative amount
        account.withdraw(-10.0);
        assert!((account.balance() - 70.0).abs() < 1e-10);

        //Withdrawing higher amount than the balance
        account.withdraw(100.0);
        assert!((account.balance() - 70.0).abs() < 1e-10);
    }

    // Add more tests here
    #[test]
    fn check_balance() {
        let account = BankAccount::new(200.0);
        assert!((account.balance() - 200.0).abs() < 1e-10);

        let mut account2 = BankAccount::new(0.0);
        account2.deposit(50.0);
        assert!((account2.balance() - 50.0).abs() < 1e-10);

        account2.withdraw(20.0);
        assert!((account2.balance() - 30.0).abs() < 1e-10);

    }
}