mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After depositing $50: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawing $30: {}", account.balance());

    account.withdraw(200.0); // Should not withdraw
    println!("After trying to withdraw $200: {}", account.balance());
}