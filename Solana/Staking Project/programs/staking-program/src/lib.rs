// Import necessary libraries and modules from the Anchor framework and Solana.
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint,  MintTo, Token, TokenAccount, Transfer},
    associated_token::AssociatedToken
};

// Declare the program ID, which is used to uniquely identify this Solana program.
declare_id!("FTBLRzteC6LV7UwhRv8KnaqMFVtcg2FWpiRo8QqkvMUP");

// Define the main program module.
#[program]
pub mod solana_stake {
    use anchor_spl::token::{mint_to, transfer};
    use super::*;

    // Initialize a mint with a specified number of decimals.
    #[allow(unused)]
    pub fn init_mint(context: Context<InitializeMint>, decimals: u8) -> Result<()> {
        Ok(())
    }

    // Airdrop tokens to an account.
    pub fn airdrop(context: Context<Airdrop>, amount: u64) -> Result<()> {
        // Generate seeds for the mint authority.
        let minting_bump = *context.bumps.get("mint_authority").unwrap();
        let mint_seeds = &["mint-authority".as_bytes(), &[minting_bump]];
        let signer = &[&mint_seeds[..]];

        // Create a context for the 'mint_to' instruction with the signer.
        let mint_to_context = context.accounts.mint_to_context().with_signer(signer);
        let result = mint_to(mint_to_context, amount);

        if result.is_err() {
            let error = result.err().unwrap();
        }

        Ok(())
    }

    // Stake tokens from an account.
    pub fn stakeToken(context: Context<Staking>, amount: u64) -> Result<()> {
        // Check if the account has sufficient tokens to stake.
        if context.accounts.account_token_account.amount < amount {
            return err!(ProgramErrors::AccountInsufficientTokenBalance);
        }

        // Create a context for the 'transfer' instruction.
        let transfer_context = context.accounts.transfer_to_vault_context();
        let result = transfer(transfer_context, amount);

        if result.is_err() {
            let error = result.err().unwrap();
        }

        // Update the stake amount in the staking account.
        context.accounts.account_stake.amount += amount;

        Ok(())
    }

    // Unstake tokens from an account.
    pub fn unstakeToken(context: Context<Staking>, amount: u64) -> Result<()> {
        // Check if there are enough staked tokens to unstake.
        if context.accounts.account_stake.amount < amount {
            return err!(ProgramErrors::UnstakingTooManyTokens);
        }

        // Generate seeds for the staking authority.
        let staking_bump = *context.bumps.get("staking_authority").unwrap();
        let staking_seeds = &["staking-authority".as_bytes(), &[staking_bump]];
        let signer = &[&staking_seeds[..]];

        // Create a context for the 'transfer' instruction with the signer.
        let transfer_context = context.accounts.transfer_to_account_context().with_signer(signer);
        let result = transfer(transfer_context, amount);

        if result.is_err() {
            let error = result.err().unwrap();
        }

        // Update the stake amount in the staking account.
        context.accounts.account_stake.amount -= amount;

        Ok(())
    }
}

// Define custom error codes for the program.
#[error_code]
pub enum ProgramErrors {
    AccountInsufficientTokenBalance,
    UnstakingTooManyTokens,
}

// Define a structure for initializing a token mint.
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct InitializeMint<'info> {
    #[account(
        init, 
        mint::authority = mint_authority,
        mint::decimals = decimals, 
        seeds = ["token-mint".as_bytes()], 
        bump, 
        payer=payer)]
    pub token_mint: Account<'info, Mint>,
    #[account(seeds = ["mint-authority".as_bytes()], bump)]
    pub mint_authority: AccountInfo<'info>,
    #[account(seeds = ["staking-authority".as_bytes()], bump)]
    pub staking_authority: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = staking_authority,
    )]
    pub staking_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Define a structure for airdropping tokens.
#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut, seeds = ["mint-authority".as_bytes()], bump)]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub account: Signer<'info>,
    #[account(
        init_if_needed,
        payer = account,
        associated_token::mint = token_mint,
        associated_token::authority = account,
    )]
    pub account_token_account: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Airdrop<'info> {
    // Create a context for the 'mint_to' instruction.
    pub fn mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.account_token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// Define a structure for staking tokens.
#[derive(Accounts)]
pub struct Staking<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,
    #[account(seeds = ["staking-authority".as_bytes()], bump)]
    pub staking_authority: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = token_mint, 
        associated_token::authority = staking_authority
    )]
    pub staking_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub account: Signer<'info>,
    #[account(
        mut, 
        associated_token::mint = token_mint, 
        associated_token::authority = account
    )]
    pub account_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = account,
        seeds = [account.key().as_ref(), "state_account".as_bytes()],
        bump,
        space = 8 + 8 + 32
    )]
    pub account_stake: Account<'info, Stake>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Define a structure for staked tokens.
#[account]
pub struct Stake {
    pub amount: u64
}

impl<'info> Staking<'info> {
    // Create a context for the 'transfer' instruction to the vault.
    pub fn transfer_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.account_token_account.to_account_info(),
            to: self.staking_token_account.to_account_info(),
            authority: self.account.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    // Create a context for the 'transfer' instruction from the vault.
    pub fn transfer_to_account_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.staking_token_account.to_account_info(),
            to: self.account_token_account.to_account_info(),
            authority: self.staking_authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
