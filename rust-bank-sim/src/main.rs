use std::io::{stdin,Stdin,Write};
use std::num::ParseIntError;

struct  Account {
    account_number: String,
    owner_name: String,
    balance: f32,
}

struct Options {

}

impl Account {
    fn deposit(&mut self, amount: f32){
        self.balance = self.balance + amount;
        println!("updated balance is {:?}", self.balance)
    }
    fn withdraw(&mut self, amount: f32) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            println!("updated balance is {:?}", self.balance);
            Ok(())
            
        } else {
            Err("Insufficient funds".to_string())
        }
    }
    fn get_balance(&self) {
        println!("Total balance in account is: {}", self.balance);
    }
}
fn main() {

    let mut account_number: String = String::new();
    stdin().read_line(&mut account_number).expect("Unable to read the bank account number");

    let mut owner_name: String = String::new();
    stdin().read_line(&mut owner_name).expect("Unable to read the owner name");

    let mut user_account = Account {
        account_number,
        owner_name,
        balance: 0.0,
    };

    loop {

        println!("");
        println!("please select below Option:");
        println!("1. deposit");
        println!("2. withdraw");
        println!("3. balance");
        println!("4. quit");


        let mut option: String = String::new();
        stdin().read_line(&mut option).expect("Unable to read the option");

        if option.trim() == "4" {
            println!("Exiting.......");
            break;
        }

        match option.trim() {
            "1" => {
                let mut amount: String = String::new();
                stdin().read_line(&mut amount).expect("Unable to read the amount");
                if let Ok(value) = amount.trim().parse::<f32>() {
                    let updated_balance = user_account.deposit(value);
                } else {
                    println!("Invalid amount entered.");
                }
                
            },
            "2" => {
                let mut amount: String = String::new();
                stdin().read_line(&mut amount).expect("Unable to read the amount");
                match amount.trim().parse::<f32>() {
                    Ok(value) if value > 0.0 => {
                        if let Err(err) = user_account.withdraw(value) {
                            println!("Error: {}", err);
                        }
                    },
                    _ => println!("Invalid amount entered."),
                } 
                
                
            }
            "3" => {
                user_account.get_balance()
            }
            &_ => {
                println!("Invalid option");
            }
        }

    }

}
