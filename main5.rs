fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: "123456".to_string(),
        holder_name: "Alice".to_string(),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: "789012".to_string(),
        holder_name: "Bob".to_string(),
        balance: 100.0,
    };

    // Test deposit method with error handling
    match account1.deposit(200.0) {
        Ok(_) => println!("Deposit successful!"),
        Err(err) => println!("Failed to deposit: {}", err),
    }

    // Test withdraw method with error handling
    match account2.withdraw(150.0) {
        Ok(_) => println!("Withdrawal successful!"),
        Err(err) => println!("Failed to withdraw: {}", err),
    }

    // Print balances
    println!(
        "Account {} ({}): Current balance: ${:.2}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Account {} ({}): Current balance: ${:.2}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposit amount must be positive.".to_string())
        } else {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {}.",
                amount, self.account_number
            );
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be positive.".to_string())
        } else if amount > self.balance {
            Err(format!(
                "Insufficient funds in account {}. Current balance: ${:.2}",
                self.account_number, self.balance
            ))
        } else {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}.",
                amount, self.account_number
            );
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
