#[cfg(test)]
mod tests {
    use crate::bank_system::{AccountType, BankSystem, Details, Gender, Status};

    use super::*;

    #[test]
    fn test_create_account() {
        let mut bank = BankSystem::new();

        //testing the number of account is zero at first
        assert!(bank.accounts.len() == 0);

        let user1 = Details {
            name: "Ola".to_string(),
            age: 19,
            address: "Ikorodu lagos".to_string(),
            gender: Gender::Male,
            occupation: "Software Engineer".to_string(),
            status: Status::Active,
            account_type: AccountType::Savings,
        };

        bank.create_account(user1);

        //asserting that our accounts has increased by 1
        assert!(bank.accounts.len() == 1);

        let user2 = Details {
            name: "Ola".to_string(),
            age: 19,
            address: "Ikorodu lagos".to_string(),
            gender: Gender::Male,
            occupation: "Software Engineer".to_string(),
            status: Status::Active,
            account_type: AccountType::Savings,
        };

        bank.create_account(user2);

        //asserting that our accounts has increased by 1 again
        assert!(bank.accounts.len() == 2);
    }

    #[test]
    fn test_unique_account_numbers() {
        let mut bank = BankSystem::new();

        //testing the number of account is zero at first
        assert!(bank.accounts.len() == 0);

        let user1 = Details {
            name: "Ola".to_string(),
            age: 19,
            address: "Ikorodu lagos".to_string(),
            gender: Gender::Male,
            occupation: "Software Engineer".to_string(),
            status: Status::Active,
            account_type: AccountType::Savings,
        };

        bank.create_account(user1);

        //asserting that our accounts has increased by 1
        assert!(bank.accounts.len() == 1);

        let user2 = Details {
            name: "Ola".to_string(),
            age: 19,
            address: "Ikorodu lagos".to_string(),
            gender: Gender::Male,
            occupation: "Software Engineer".to_string(),
            status: Status::Active,
            account_type: AccountType::Savings,
        };

        bank.create_account(user2);

        //asserting that our accounts has increased by 1 again
        assert!(bank.accounts.len() == 2);

        // println!("bank.accounts[0].balance {}", bank.accounts[0].balance);

        assert!(bank.accounts[0].account_number != bank.accounts[1].account_number);
    }

    #[test]
    pub fn test_zero_balance_on_creation() {
        let mut bank = BankSystem::new();

        //testing the number of account is zero at first
        assert!(bank.accounts.len() == 0);

        let user1 = Details {
            name: "Ola".to_string(),
            age: 19,
            address: "Ikorodu lagos".to_string(),
            gender: Gender::Male,
            occupation: "Software Engineer".to_string(),
            status: Status::Active,
            account_type: AccountType::Savings,
        };

        bank.create_account(user1);
        assert!(bank.accounts[0].balance == 0.0);
    }
    // #[test]
    // pub fn test_account_balance() {
    //      let mut bank = BankSystem::new();

    //     //testing the number of account is zero at first
    //     assert!(bank.accounts.len() == 0);

    //     let user1 = Details {
    //         name: "Ola".to_string(),
    //         age: 19,
    //         address: "Ikorodu lagos".to_string(),
    //         gender: Gender::Male,
    //         occupation: "Software Engineer".to_string(),
    //         status: Status::Active,
    //         account_type: AccountType::Savings,
    //     };

    //     bank.create_account(user1);

    //     assert_eq!(bank.accounts[0].balance, 123.0);
    // }
}
