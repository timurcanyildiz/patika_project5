pub trait Account {
    fn deposit(&mut self, amount: f64)-> Result<f64,String>;
    fn withdraw(&mut self, amount: f64)-> Result<f64,String>;
    fn balance(&mut self);

}

pub struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64
}

impl Account for BankAccount{
     fn deposit(&mut self, amount: f64)  -> Result<f64, String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(self.balance)  
            
            // println!("You deposited {}. New balance is: {}", amount,self.balance),
        }
        else {
            Err(String::from("You cannot deposit negative value! Please try again."))
        }
        
        
        
    }
    fn balance(&mut self) {
        println!("Current balance in account number: {} is: {}",self.account_number, self.balance);
    }
    fn withdraw(&mut self, amount: f64) -> Result<f64,String> {
        
        if amount > self.balance {
           Err(String::from("Insufficient funds. Cannot withdraw more than your balance."))
        }
        else {
            self.balance -= amount;
            Ok(self.balance)
        }

    }
}

fn main() {
  
  let mut first_account = BankAccount{account_number:1015226,holder_name:String::from("Halil HASAR"),balance:4412.65};
  let mut second_account = BankAccount{account_number:3567881,holder_name:String::from("Timurcan YILDIZ"),balance:10455.20};

  first_account.balance();
    match first_account.deposit(255.0) {
        Ok(_) => first_account.balance(),
        Err(error) => println!("Deposit error: {}", error),
    }
    match first_account.withdraw(400.50) {
        Ok(_) => first_account.balance(),
        Err(error) => println!("Withdraw error: {}", error),
    }
    match first_account.withdraw(10000.00) {
        Ok(_) => first_account.balance(),
        Err(error) => println!("Withdraw error: {}", error),
    }

    second_account.balance();
    match second_account.withdraw(20000.25) {
        Ok(_) => second_account.balance(),
        Err(error) => println!("Withdraw error: {}", error),
    }
    match second_account.withdraw(320.0) {
        Ok(_) => second_account.balance(),
        Err(error) => println!("Withdraw error: {}", error),
    }
    match second_account.deposit(-15.25) {
        Ok(_) => second_account.balance(),
        Err(error) => println!("Deposit error: {}", error),
    }
}



