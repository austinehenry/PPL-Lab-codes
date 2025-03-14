// Define the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f32,
}

// Implement methods for BankAccount
impl BankAccount {
    // Method to create a new bank account
    fn new(account_number: u32, holder_name: String, balance: f32) -> BankAccount {
        BankAccount {
            account_number,
            holder_name,
            balance,
        }
    }

    // Method to deposit money
    fn deposit(&mut self, amount: f32) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited: {:.2}", amount);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    // Method to withdraw money (with balance check)
    fn withdraw(&mut self, amount: f32) {
        if amount > 0.0 {
            if self.balance >= amount {
                self.balance -= amount;
                println!("Withdrawn: {:.2}", amount);
            } else {
                println!("Insufficient balance. Available balance: {:.2}", self.balance);
            }
        } else {
            println!("Withdrawal amount must be positive.");
        }
    }

    // Method to display account details
    fn display(&self) {
        println!("Account Number: {}", self.account_number);
        println!("Holder Name: {}", self.holder_name);
        println!("Balance: {:.2}", self.balance);
    }
}

fn main() {
    // Create a new BankAccount instance
    let mut account = BankAccount::new(101, String::from("Alice"), 500.0);

    // Display initial account details
    account.display();

    // Perform transactions
    account.deposit(200.0);
    account.withdraw(100.0);
    account.withdraw(700.0); // Should display an error message

    // Display final account details
    account.display();
}
