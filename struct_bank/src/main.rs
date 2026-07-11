struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance > amount {
            self.balance -= amount;
        }
        println!("Not Enough Balance");
    }
    fn display(&self) {
        println!("Owner Name = {}", self.owner);
        println!("Balance = {}", self.balance);
    }
}

fn main() {
    //Create Bank Account
    let mut bank_account: BankAccount = BankAccount {
        owner: String::from("Jeel"),
        balance: 13000.00,
    };

    //Print details

    bank_account.display();
    //Deposit
    bank_account.deposit(1000.00);
    bank_account.display();
    bank_account.withdraw(1000.00);
    bank_account.display();
}
