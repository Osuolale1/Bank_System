use rand::Rng;
use std::collections::HashSet;

// pub mod bankSystem;
use crate::bank_system::*;

impl BankSystem {
    pub fn new() -> BankSystem {
        BankSystem {
            accounts: Vec::new(),
            used_account_numbers: HashSet::new(),
        }
    }

    // Generate unique 10-digit account number
    fn generate_account_number(&mut self) -> String {
        let mut rng = rand::thread_rng();

        loop {
            // Generate 10-digit number (1000000000 to 9999999999)
            let account_number: u64 = rng.gen_range(1_000_000_000..=9_999_999_999);
            let account_str = account_number.to_string();

            // Check if this number is already used
            if !self.used_account_numbers.contains(&account_str) {
                self.used_account_numbers.insert(account_str.clone());
                return account_str;
            }
        }
    }

    pub fn create_account(&mut self, details: Details) -> String {
        let account_number = self.generate_account_number();

        let account = Account {
            account_number: account_number.clone(),
            details,
            balance: 0.0, // New accounts start with 0 balance
        };

        self.accounts.push(account);
        account_number
    }

    pub fn find_account(&self, account_number: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|account| account.account_number == account_number)
    }

    pub fn create_account_interactive(&mut self) {
        println!("\n=== Create New Account ===");

        // Get user details
        println!("Enter full name: ");
        let name = get_input();

        println!("Enter age: ");
        let age: u32 = loop {
            match get_input().parse() {
                Ok(age) if age >= 18 => break age,
                Ok(_) => println!("Age must be 18 or older. Please enter again: "),
                Err(_) => println!("Invalid age. Please enter a number: "),
            }
        };

        println!("Enter address: ");
        let address = get_input();

        println!("Enter gender (1 for Male, 2 for Female): ");
        let gender = loop {
            match get_input().trim() {
                "1" => break Gender::Male,
                "2" => break Gender::Female,
                _ => println!("Invalid choice. Enter 1 for Male or 2 for Female: "),
            }
        };

        println!("Enter occupation: ");
        let occupation = get_input();

        println!("Select account type:");
        println!("1. Student");
        println!("2. Personal");
        println!("3. Savings");
        println!("4. Current");
        println!("5. Corporate");

        let account_type = loop {
            match get_input().trim() {
                "1" => break AccountType::Student,
                "2" => break AccountType::Personal,
                "3" => break AccountType::Savings,
                "4" => break AccountType::Current,
                "5" => break AccountType::Corporate,
                _ => println!("Invalid choice. Please select 1-5: "),
            }
        };

        let details = Details {
            name,
            age,
            address,
            gender,
            occupation,
            status: Status::Active, // New accounts are active by default
            account_type,
        };

        let account_number = self.create_account(details);

        println!("\n✅ Account created successfully!");
        println!("Your account number is: {}", account_number);
        println!("Please keep this number safe for future transactions.");
    }

    pub fn view_account_interactive(&self) {
        println!("\n=== View Account Details ===");
        println!("Enter account number: ");
        let account_number = get_input();

        match self.find_account(&account_number) {
            Some(account) => {
                println!("\n=== Account Details ===");
                println!("Account Number: {}", account.account_number);
                println!("Name: {}", account.details.name);
                println!("Age: {}", account.details.age);
                println!("Address: {}", account.details.address);
                println!("Gender: {:?}", account.details.gender);
                println!("Occupation: {}", account.details.occupation);
                println!("Account Type: {:?}", account.details.account_type);
                println!("Status: {:?}", account.details.status);
                println!("Balance: ₦{:.2}", account.balance);
            }
            None => println!("❌ Account not found. Please check the account number."),
        }
    }

    pub fn list_all_accounts(&self) {
        println!("\n=== All Accounts ===");
        if self.accounts.is_empty() {
            println!("No accounts found.");
            return;
        }

        for (index, account) in self.accounts.iter().enumerate() {
            println!(
                "{}. {} - {} ({:?})",
                index + 1,
                account.account_number,
                account.details.name,
                account.details.account_type
            );
        }
    }

    pub fn account_balance(&self) {
        println!("\n=== View Account Balance ===");
        println!("Enter account number: ");
        let account_number = get_input();

        match self.find_account(&account_number) {
            Some(account) => {
                println!("Balance: ₦{:.2}", account.balance);
            }
            None => println!("❌ Account not found. Please check the account number."),
        }
    }

    pub fn deposit_interactive(&mut self) {
        println!("\n=== Deposit ===");
        let account_number = get_input();

        println!("Enter Amount");
        let amount_input = get_input();

        let amount = match amount_input.parse::<f64>() {
            Ok(amount) if amount > 0.1 => amount,
            Ok(_) => {
                println!("Value must be greater than 0.1");
                return;
            }
            Err(_) => {
                println!("Invalid number, Enter a  valid number");
                return;
            }
        };

        match self
            .accounts
            .iter_mut()
            .find(|account| account.account_number == account_number)
        {
            Some(account) => match account.details.status {
                Status::Active => {
                    account.balance += amount;
                    println!("✅ Successfully deposited ₦{:.2}", amount);
                    println!("New balance: ₦{:.2}", account.balance);
                }
                Status::Closed => {
                    println!(
                        "😩 Your account has been closed, Visit our nearest bank to re-open your account"
                    );
                }
                Status::Dormant => {
                    println!(
                        "😩 Your account has entered Dormant, Visit our nearest bank to re-activate your account"
                    );
                }
            },
            None => {
                println!("Account not foumd")
            }
        }
    }

    pub fn withdraw_interactive(&mut self) {
        println!("\n=== Withdraw ===");
        let account_number = get_input();

        println!("Enter Amount");
        let amount_input = get_input();

        let amount = match amount_input.parse::<f64>() {
            Ok(amount) if amount > 0.1 => amount,
            Ok(_) => {
                println!("Value must be greater than 0.1");
                return;
            }
            Err(_) => {
                println!("Invalid number, Enter a valid number");
                return;
            }
        };

        match self
            .accounts
            .iter_mut()
            .find(|account| account.account_number == account_number)
        {
            Some(account) => match account.details.status {
                Status::Active => {
                    if account.balance < amount {
                        println!("insufficient balance");
                        return;
                    }
                    account.balance -= amount;
                    println!("✅ Successfully withdraw ₦{:.2}", amount);
                    println!("New balance: ₦{:.2}", account.balance);
                }
                Status::Closed => {
                    println!(
                        "😩 Your account has been closed, Visit our nearest bank to re-open your account"
                    );
                }
                Status::Dormant => {
                    println!(
                        "😩 Your account has entered Dormant, Visit our nearest bank to re-activate your account"
                    );
                }
            },
            None => {
                println!("Account not foumd")
            }
        }
    }
}
