// Import necessary external crates and Solana program dependencies.
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;

// Define a struct `CalAcct` that represents the calculator account with a single field `result`.
// This struct will be serialized and stored in a Solana account.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalAcct {
    pub result: f64,
}

// Define the entry point for the Solana program, named `process_instruction`.
entrypoint!(process_instruction);

// The main function that implements the calculator logic.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    info_data: &[u8],
) -> ProgramResult {
    // Create an iterator for the provided accounts.
    let accounts_iter = &mut accounts.iter();
    // Retrieve the first account from the iterator.
    let account = next_account_info(accounts_iter)?;

    // Check if the provided account's owner matches the program's ID.
    if account.owner != program_id {
        // If not, initialize the account data with a default `CalAcct` instance (result set to 0.0).
        let data = &mut account.data.borrow_mut();
        let calculator_account = CalAcct { result: 0.0 };
        calculator_account
            .serialize(&mut &mut data[..])
            .map_err(|_| ProgramError::InvalidAccountData)?;
    }

    // Deserialize the `CalAcct` instance from the account data.
    let mut calculator_account = {
        let data = account.data.borrow();
        CalAcct::try_from_slice(&data[..])?
    };

    // Check if the instruction data is at least 1 byte long.
    if info_data.len() < 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the operation code (0 for addition, 1 for subtraction) from the instruction data.
    let operation = info_data[0];

    // Perform the appropriate operation based on the operation code.
    match operation {
        0 => {
            // Check if there are enough bytes in the instruction data for addition.
            if info_data.len() < 17 {
                return Err(ProgramError::InvalidInstructionData);
            }
            // Extract two 8-byte chunks from the instruction data and convert them to f64 numbers.
            let num1_bytes = &info_data[1..9];
            let num2_bytes = &info_data[9..17];
            let num1 = f64::from_le_bytes(num1_bytes.try_into().unwrap());
            let num2 = f64::from_le_bytes(num2_bytes.try_into().unwrap());
            // Call the `perform_addition` function to perform the addition operation.
            perform_addition(&mut calculator_account, num1, num2);
        }
        1 => {
            // Check if there are enough bytes in the instruction data for subtraction.
            if info_data.len() < 17 {
                return Err(ProgramError::InvalidInstructionData);
            }
            // Extract two 8-byte chunks from the instruction data and convert them to f64 numbers.
            let num1_bytes = &info_data[1..9];
            let num2_bytes = &info_data[9..17];
            let num1 = f64::from_le_bytes(num1_bytes.try_into().unwrap());
            let num2 = f64::from_le_bytes(num2_bytes.try_into().unwrap());
            // Call the `perform_subtraction` function to perform the subtraction operation.
            perform_subtraction(&mut calculator_account, num1, num2);
        }
        _ => {
            // If the operation code is not 0 or 1, return an error.
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    // Serialize the updated `CalAcct` instance and store it back in the account data.
    calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // Return a successful program result.
    Ok(())
}

// Helper function to perform addition and update the `result` field of the `CalAcct` instance.
pub fn perform_addition(acct: &mut CalAcct, num1: f64, num2: f64) {
    acct.result = num1 + num2;
}

// Helper function to perform subtraction and update the `result` field of the `CalAcct` instance.
pub fn perform_subtraction(acct: &mut CalAcct, num1: f64, num2: f64) {
    acct.result = num1 - num2;
}
