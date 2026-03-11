//mod rectangle;
//use rectangle::Rectangle;

mod bank_account;
use bank_account::BankAccount;

fn main() {
    
   /*
    let rect = Rectangle::new(5, 10);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Is square? {}", rect.is_square());

    let rect1 = Rectangle::new(8, 7);
    let rect2 = Rectangle::new(5, 1);
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
    */
    //create new account
    let mut account = BankAccount::new(100.50);
    println!("New account balance: ${:?}", account);

    //deposit to account
    let depo = 74.5;
    println!("Amount depositing: ${}", depo);
    account.deposit(depo);
    println!("Updated balance after deposit: ${:?}", account.balance());

    //withdraw from account
    let withdraw_amount = 55.0;
    println!("Amount withdrawing: ${}", withdraw_amount);
    account.withdraw(withdraw_amount);
    println!("Updated balance after withdraw: ${:?}", account.balance());

    println!(""); //spacer

    //check balance after deposit & withdraw
    println!("Current balance: ${:?}", account.balance());
    
    println!(""); //spacer
    
    //deposit negative amount
    let neg_depo = -4.5;
    println!("Amount depositing: ${}", neg_depo);
    account.deposit(neg_depo);
    println!("Updated balance after deposit: ${:?}", account.balance());

    //withdraw negative amount
    let neg_withdraw_amount = -5.0;
    println!("Amount withdrawing: ${}", neg_withdraw_amount);
    account.withdraw(neg_withdraw_amount);
    println!("Updated balance after withdraw: ${:?}", account.balance());

    println!(""); //spacer
    //print current balance
    println!("Current balance: ${:?}", account.balance());

    println!(""); //spacer

    //withdraw full balance
    let full_balance:f64 = account.balance();
    println!("Amount withdrawing: ${}", full_balance);
    account.withdraw(full_balance);
    println!("updated balance: ${:?}", account.balance());

    println!(""); //spacer

    //withdraw from empty balance
    let withdraw_amount = 1.5;
    println!("Amount withdrawing: ${}", withdraw_amount);
    account.withdraw(withdraw_amount);
    //println!("Updated balance after withdraw: {:?}", account.balance());

    println!("Current balance: ${:?}", account.balance());
    
}
