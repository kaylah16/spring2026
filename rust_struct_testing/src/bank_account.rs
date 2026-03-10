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
            self.balance = self.balance + amount;
        }

    }

    pub fn withdraw(&mut self, amount: f64) {
        // decrease balance
            // remain unchanged when amount >= balance or amount is negative
        if amount > self.balance || amount <= 0.0{ //check amount is negative or greater than balance
            println!("Given amount is negative or greater than current balance, ignored");
            return;
        }
        if amount < self.balance {
            self.balance = self.balance - amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // return current balance
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

    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.5);
        let withdraw_amount = 5.5;
        account.deposit(withdraw_amount);
    }

    // Add more tests here
    
    #[test]
    fn test_check_balance() {
        //check what the current balance is
        let mut account = BankAccount::new(100.5);

        let add = 6.35;
        account.deposit(add);
        account.balance();
    }
    
}
