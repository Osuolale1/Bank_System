use std::collections::HashSet;
use std::io;

#[derive(Debug, Clone)]
pub struct Account {
    pub account_number: String,
    pub details: Details,
    pub balance: f64,
}

#[derive(Debug, Clone)]
pub struct Details {
    pub name: String,
    pub age: u32,
    pub address: String,
    pub gender: Gender,
    pub occupation: String,
    pub status: Status,
    pub account_type: AccountType,
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    Student,
    Personal,
    Savings,
    Current,
    Corporate,
}

#[derive(Debug, Clone)]
pub enum Status {
    Active,
    Dormant,
    Closed,
}

pub struct BankSystem {
    pub accounts: Vec<Account>,
    pub used_account_numbers: HashSet<String>,
}

// Helper function to get user input
pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Display implementations for better formatting
impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Student => write!(f, "Student Account"),
            AccountType::Personal => write!(f, "Personal Account"),
            AccountType::Savings => write!(f, "Savings Account"),
            AccountType::Current => write!(f, "Current Account"),
            AccountType::Corporate => write!(f, "Corporate Account"),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Active => write!(f, "Active"),
            Status::Dormant => write!(f, "Dormant"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}
