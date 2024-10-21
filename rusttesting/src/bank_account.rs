#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method

        //create a new account
        BankAccount { balance: initial_balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method

        //should increase the balance. ignore the operation if the amout is negative

        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method

        //should decrease the balance. if the amount is greater or negative, the balance should remain unchanged
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method

        //should return the current balance
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
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money

        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);

    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money

        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert_eq!(account.balance(), 50.0);
    }

    // Add more tests here

    #[test]
    fn checking_balance(){

        // Write a test for checking the balance
        assert_eq!(BankAccount::new(100.0).balance(), 100.0);
    }


    //edge cases
    #[test]
    fn deposit_negative_amount(){
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn withdraw_negative_amount(){
        let mut account = BankAccount::new(100.0);
        account.withdraw(-50.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn withdraw_more_than_balance(){
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);
        assert_eq!(account.balance(), 100.0);
    }


}