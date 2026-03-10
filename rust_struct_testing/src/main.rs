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
    
    let mut account = BankAccount::new(100.50);
    println!("New account balance: {:?}", account);

    let depo = -4.5;
    account.deposit(depo);
    println!("Updated balance after deposit: {:?}", account.balance());

    let withdraw_amount = -5.0;
    account.withdraw(withdraw_amount);
    println!("Updated balance after withdraw: {:?}", account.balance());

    println!("Current balance: {:?}", account.balance());
}
