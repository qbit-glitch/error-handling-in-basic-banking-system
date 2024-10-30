use std::fmt;

fn main() {
    let mut bank_account_1 = BankAccount{
        account_number: 8976345678914365,
        holder_name: "Jazmin".to_string(),
        balance: 1900.60, 
    };

    let mut bank_account_2 = BankAccount{
        account_number: 8769354287651034,
        holder_name: "Alice".to_string(),
        balance: 1400.00,
    };

    let transfer_amount: f64 = 400.50;
    println!("Account-1 Balance: {}, Account-2 Balance: {} ", bank_account_1.balance, bank_account_2.balance);
    match bank_account_1.deposit(transfer_amount){
        Ok(()) => println!("Money Deposited Successfully! into Account Number: {}", bank_account_1.account_number),
        Err(e) => println!("Error: {} for depositing on Account Number: {}", e, bank_account_1.account_number),
    }


    match bank_account_2.withdraw(transfer_amount) {
        Ok(()) => println!("Money Withdrawn Successfully! from Account Number: {}", bank_account_2.account_number),
        Err(e) => println!("Error: {} for withdrawal from Account Number: {}", e, bank_account_2.account_number),
    }
    println!("Account-1 Balance: {}, Account-2 Balance: {} ", bank_account_1.balance(), bank_account_2.balance());

}


trait Account{
    fn deposit(&mut self, deposit:f64) -> Result<(), String>;
    fn withdraw(&mut self, withdraw:f64) -> Result<(), String>;
    fn balance(&mut self) -> f64;
}

struct BankAccount{
    account_number : i64,
    holder_name: String,
    balance : f64,
}

impl Account for BankAccount{
    fn deposit(&mut self, deposit: f64) -> Result<(), String>{
        let deposit_limit = 2000.0;        

        if (self.balance + deposit) < deposit_limit {
            self.balance += deposit;
            println!("{}",self.balance);
            Ok(())
        } else {
            Err("Deposit Limit Exceeded".to_string())
        }
    }

    fn withdraw(&mut self, withdraw: f64) -> Result<(), String>{
        let min_bal: f64 = 1000.00;
        if (self.balance - withdraw) >= min_bal
            {   self.balance -= withdraw;
                Ok(())
            }
        else{
            Err("Balance below 1000. Cannor withdraw money".to_string())
        }
    }

    fn balance(&mut self) -> f64{
        self.balance
    }
}