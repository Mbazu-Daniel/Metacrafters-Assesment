use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    /// Result of the sum operation
    pub sum_result: u32,
    /// Result of the difference operation
    pub diff_result: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the calculator program was loaded into
    accounts: &[AccountInfo], // Accounts used by the program
    instruction_data: &[u8], // Input data containing two numbers and operation choice
) -> ProgramResult {
    msg!("Calculator Rust program entrypoint");

    // Ensure the instruction data is the correct size
    if instruction_data.len() != 12 {
        msg!("Invalid instruction data size");
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the input data
    let num1 = u32::from_le_bytes(instruction_data[0..4].try_into().unwrap());
    let num2 = u32::from_le_bytes(instruction_data[4..8].try_into().unwrap());
    let operation = u32::from_le_bytes(instruction_data[8..12].try_into().unwrap());

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the calculator account to store the results
    let calculator_account = next_account_info(accounts_iter)?;

    // The calculator account must be owned by the program
    if calculator_account.owner != program_id {
        msg!("Calculator account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Perform the requested operation
    let mut calculator_data = CalculatorAccount::try_from_slice(&calculator_account.data.borrow())?;

    match operation {
        0 => {
            // Calculate the sum
            calculator_data.sum_result = num1 + num2;
            msg!("Sum result: {}", calculator_data.sum_result);
        }
        1 => {
            // Calculate the difference
            if num1 >= num2 {
                calculator_data.diff_result = num1 - num2;
                msg!("Difference result: {}", calculator_data.diff_result);
            } else {
                msg!("Invalid difference operation: num1 is less than num2");
                return Err(ProgramError::InvalidArgument);
            }
        }
        _ => {
            msg!("Invalid operation choice");
            return Err(ProgramError::InvalidArgument);
        }
    }

    // Serialize and store the updated calculator data
    calculator_data.serialize(&mut &mut calculator_account.data.borrow_mut()[..])?;

    Ok(())
}

// Tests for the calculator program
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_calculator() {
        let program_id = Pubkey::default();
        let calculator_key = Pubkey::default();
        let mut lamports = 0;
        let mut calculator_data = vec![0; mem::size_of::<CalculatorAccount>()];
        let owner = Pubkey::default();
        let calculator_account = AccountInfo::new(
            &calculator_key,
            false,
            true,
            &mut lamports,
            &mut calculator_data,
            &owner,
            false,
            Epoch::default(),
        );

        let num1 = 42;
        let num2 = 15;
        let operation = 0; // 0 for sum
        let instruction_data = [num1.to_le_bytes(), num2.to_le_bytes(), operation.to_le_bytes()]
            .concat();

        let accounts = vec![calculator_account];

        assert_eq!(
            CalculatorAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .sum_result,
            0
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        assert_eq!(
            CalculatorAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .sum_result,
            num1 + num2
        );

        // Test the difference operation
        let operation = 1; // 1 for difference
        let instruction_data = [num1.to_le_bytes(), num2.to_le_bytes(), operation.to_le_bytes()]
            .concat();

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        assert_eq!(
            CalculatorAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .diff_result,
            num1 - num2
        );
    }
}
