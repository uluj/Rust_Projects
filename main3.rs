fn main(){

}
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!(
            "Deposited ${:.2} into account {}.",
            amount, self.account_number
        );
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}.",
                amount, self.account_number
            );
        } else {
            println!(
                "Insufficient funds in account {}. Current balance: ${:.2}",
                self.account_number, self.balance
            );
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}