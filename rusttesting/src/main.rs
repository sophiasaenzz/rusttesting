mod bank_account;
use bank_account::BankAccount;


fn main() {
    //println!("Hello, world!");

    // Create a new account
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

}
