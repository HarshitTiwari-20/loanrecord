#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Loan {
    pub lender: Address,
    pub borrower: Address,
    pub amount: i128,
    pub repaid: i128,
    pub is_active: bool,
}

#[contract]
pub struct LoanRecordContract;

#[contractimpl]
impl LoanRecordContract {
    /// Create a new loan record.
    /// 
    /// # Arguments
    /// * `env` - The Execution Environment
    /// * `loan_id` - A unique integer ID for the loan
    /// * `lender` - The address of the lender 
    /// * `borrower` - The address of the borrower
    /// * `amount` - The total loan amount
    pub fn create_loan(env: Env, loan_id: u32, lender: Address, borrower: Address, amount: i128) {
        lender.require_auth();

        if amount <= 0 {
            panic!("Loan amount must be greater than zero");
        }

        if env.storage().persistent().has(&loan_id) {
            panic!("Loan ID already exists");
        }

        let loan = Loan {
            lender,
            borrower,
            amount,
            repaid: 0,
            is_active: true,
        };

        env.storage().persistent().set(&loan_id, &loan);
    }

    /// Repay a portion of the loan.
    ///
    /// # Arguments
    /// * `env` - The Execution Environment
    /// * `loan_id` - The unique integer ID of the loan being repaid
    /// * `borrower` - The address of the borrower (must match the loan's borrower)
    /// * `amount` - The amount being repaid
    pub fn repay_loan(env: Env, loan_id: u32, borrower: Address, amount: i128) {
        borrower.require_auth();

        if amount <= 0 {
            panic!("Repayment amount must be greater than zero");
        }

        let mut loan: Loan = env.storage().persistent().get(&loan_id).expect("Loan not found");

        if loan.borrower != borrower {
            panic!("Only the original borrower can execute repayments");
        }

        if !loan.is_active {
            panic!("Loan is already fully repaid or inactive");
        }

        loan.repaid += amount;

        // Automatically mark the loan as inactive once fully repaid
        if loan.repaid >= loan.amount {
            loan.is_active = false;
        }

        env.storage().persistent().set(&loan_id, &loan);
    }

    /// Retrieve the details of a loan.
    pub fn get_loan(env: Env, loan_id: u32) -> Loan {
        env.storage().persistent().get(&loan_id).expect("Loan not found")
    }
}
