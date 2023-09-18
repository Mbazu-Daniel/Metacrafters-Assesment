use anchor_lang::prelude::*;

// Declare the ID for the program. This is a unique identifier used to interact with the program on the Solana blockchain.
declare_id!("5AUyioSxkS2bYe8fZUdvGJphwCJv42SYCThMMTef39eH");

// Define the program module.
#[program]
pub mod counter_program {
    use super::*;

    // Function to create a new counter account.
    pub fn create(ctx: Context<Create>) -> Result<()> {
        // Set the authority of the counter account to the provided authority's key.
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        // Initialize the count to 0.
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    // Function to increment the count in the counter account.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Increase the count by 1.
        ctx.accounts.counter.count += 1;
        Ok(())
    }

    // Function to decrement the count in the counter account.
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        // Decrease the count by 1.
        ctx.accounts.counter.count -= 1;
        Ok(())
    }    
}

// Define the Create accounts structure.
#[derive(Accounts)]
pub struct Create<'info> {
    // Define the counter account. It is initialized, and the payer is set to the authority.
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub counter: Account<'info, Counter>,
    // Define the authority account, which is mutable.
    #[account(mut)]
    pub authority: Signer<'info>,
    // Define the system program account.
    pub system_program: Program<'info, System>
}

// Define the Increment accounts structure.
#[derive(Accounts)]
pub struct Increment<'info> {
    // Define the counter account, which is mutable and must have the authority.
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    // Define the authority account.
    pub authority: Signer<'info>,
}

// Define the Decrement accounts structure.
#[derive(Accounts)]
pub struct Decrement<'info> {
    // Define the counter account, which is mutable and must have the authority.
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    // Define the authority account.
    pub authority: Signer<'info>,
}

// Define the Counter account.
#[account]
pub struct Counter {
    // Define the authority public key associated with the counter.
    pub authority: Pubkey,
    // Define the count stored in the counter account.
    pub count: u64,
}
