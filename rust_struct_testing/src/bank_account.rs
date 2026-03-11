#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // create a new account
        BankAccount {balance: initial_balance}
        
    }

    pub fn deposit(&mut self, amount: f64) {
        // increase balance
            //ignore if given amount is negative
        //let mut update_amount = 0; //updated balance
        //amount is money recieved

        if amount <= 0.0 { //if amount is negative, don't update
            println!("Given amount is negative, ignored");
            return;
        }
        if amount > 0.0 { //if amount is more than $0.0, add to balance
            self.balance += amount;
        } //mostly redundant to have if statement, but still better to check

    }

    pub fn withdraw(&mut self, amount: f64) {
        // decrease balance
            // remain unchanged when amount >= balance or amount is negative
        if amount > self.balance { //check amount is negative or greater than balance
            println!("Given amount is greater than current balance, ignored");
            return;
        }
        else if amount <= 0.0 {
            println!("Given amount is negative, ignored");
            return;
        }
        //if amount < self.balance {
            self.balance -= amount;
        
    }

    pub fn balance(&self) -> f64 {
        // check balance
        return self.balance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        BankAccount::new(100.5);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.5);
        let deposit_amount = 5.5;
        account.deposit(deposit_amount);
        assert_eq!(account.balance(), 106.0);

    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.5);
        let withdraw_amount = 5.5;
        account.withdraw(withdraw_amount);
        assert_eq!(account.balance(), 95.0);
    }

    // Add more tests here
    
    #[test]
    fn test_check_balance() {
        //check what the current balance is
        let mut account = BankAccount::new(100.5);

        let add = 6.35;
        account.deposit(add);
        account.balance();
        assert_eq!(account.balance(), 106.85)
    }

    #[test]
    fn test_negative_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.5);
        let deposit_amount = -10.5;
        account.deposit(deposit_amount);

    }

    #[test]
    fn test_negative_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.5);
        let withdraw_amount = -25.5;
        account.withdraw(withdraw_amount);
    }

    #[test]
    fn test_past_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.5);
        let withdraw_amount = 205.5;
        account.withdraw(withdraw_amount);
    }
    
}
