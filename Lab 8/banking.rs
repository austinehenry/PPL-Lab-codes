use std::cell::RefCell;

struct BankAccount {
    account_number: String,
    owner_name: String,
    balance: RefCell<f64>,
}

impl BankAccount {
    // Constructor for a new bank account
    fn new(account_number: &str, owner_name: &str, balance: f64) -> Self {
        BankAccount {
            account_number: account_number.to_string(),
            owner_name: owner_name.to_string(),
            balance: RefCell::new(balance),
        }
    }

    // View the balance with an immutable borrow
    fn view_balance(&self) -> f64 {
        println!("Account Number: {}, Owner: {}", self.account_number, self.owner_name);
        *self.balance.borrow() // Borrow the balance immutably
    }

    // Deposit money into the account (mutable borrow)
    fn deposit(&self, amount: f64) {
        let mut balance = self.balance.borrow_mut(); // Mutably borrow the balance
        *balance += amount; // Modify the balance
        println!("Deposited {} to account {}", amount, self.account_number);
    }

    // Withdraw money from the account (mutable borrow)
    fn withdraw(&self, amount: f64) -> Result<(), String> {
        let mut balance = self.balance.borrow_mut(); // Mutably borrow the balance
        if *balance >= amount {
            *balance -= amount; // Modify the balance
            println!("Withdrew {} from account {}", amount, self.account_number);
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }
}

fn main() {
    // Create a new bank account
    let account = BankAccount::new("123456", "John Doe", 1000.0);

    // Multiple users viewing the balance (immutable borrow)
    println!("Balance before deposit: {}", account.view_balance());
    println!("Balance before withdrawal: {}", account.view_balance());

    // Depositing money into the account (mutable borrow)
    account.deposit(500.0);
    println!("Balance after deposit: {}", account.view_balance());

    // Withdrawing money from the account (mutable borrow)
    match account.withdraw(300.0) {
        Ok(_) => println!("Balance after withdrawal: {}", account.view_balance()),
        Err(e) => println!("{}", e),
    }

    // Attempting to withdraw more than the balance (insufficient funds)
    match account.withdraw(2000.0) {
        Ok(_) => println!("Balance after withdrawal: {}", account.view_balance()),
        Err(e) => println!("{}", e),
    }
}
