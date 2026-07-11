struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&self, &amount: &String) {
        self.balance += amount;
    }
}

fn main() {
    //Create Bank Account
    let bank_account:BankAccount = BankAccount{
        owner:String::from("Jeel"),
        balance:13000.00
    }
    //
}

fn print_bank_details(&bank_account:BankAccount)
{
    println!("Owner Name = {}",bank_account.owner);
    println!("Balance = {}",bank_account.balance)
}
