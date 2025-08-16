// Bank Account Simulator

// Deposit, withdraw, check balance.

// Handle overdraft errors with proper Result types.
// Concepts: Traits (e.g. AccountOps), modular CLI routing.

use std::io;

pub mod bank_system;
pub mod functions;
pub mod test;

use crate::bank_system::*;

fn main() {
    let mut bank = BankSystem::new();

    println!("=== Nigerian Bank Account Creation System ===");

    loop {
        println!("\n1. Create New Account");
        println!("2. View Account Details");
        println!("3. List All Accounts");
        println!("4. View Account Balance");
        println!("5. Deposit");
        println!("6. Withdraw");
        println!("7. Exit");
        println!("Choose an option: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => bank.create_account_interactive(),
            "2" => bank.view_account_interactive(),
            "3" => bank.list_all_accounts(),
            "4" => bank.account_balance(),
            "5" => bank.deposit_interactive(),
            "6" => bank.withdraw_interactive(),
            "7" => {
                println!("Thank you for using Nigerian Bank System!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
