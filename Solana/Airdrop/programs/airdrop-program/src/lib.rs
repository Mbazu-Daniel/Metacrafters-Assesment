// Import necessary modules and dependencies
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Mint, MintTo};

// Declare the program ID
declare_id!("FTBLRzteC6LV7UwhRv8KnaqMFVtcg2FWpiRo8QqkvMUP");

// Define the program module
#[program]
pub mod airdrop_program {
    // Import items from the parent module
    use super::*;

    // Initialize the mint
    pub fn initialize_mint(context: Context<InitializeMint>, _decimals: u8) -> Result<()> {
        // Return success
        Ok(())
    }

    // Perform an airdrop
    pub fn airdrop(context: Context<Airdrop>, amount: u64) -> Result<()> {
        // Get the mint authority bump
        let mint_authority_bump = *context.bumps.get("mint_authority").unwrap();
        // Create seeds for the mint authority
        let mint_authority_seeds = &["mint-authority".as_bytes(), &[mint_authority_bump]];
        // Create a signer for the mint authority
        let mint_authority_signer = &[&mint_authority_seeds[..]];

        // Create a context for the mint-to operation
        let mint_context = context.accounts.mint_to_context().with_signer(mint_authority_signer);
        // Call the token::mint_to function to perform the mint-to operation
        token::mint_to(mint_context, amount)?;

        // Return success
        Ok(())
    }
}

// Define account structures
#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct InitializeMint<'info> {
    // Initialize the token mint account
    #[account(
        init,
        seeds = ["token-mint".as_bytes()], // Seed for the token mint account
        bump, // Bump seed
        payer = payer, // Payer of the transaction
        mint::authority = mint_authority, // Mint authority account
        mint::decimals = decimals // Decimals for the mint
    )]
    pub token_mint: Account<'info, Mint>, // Token mint account
    // Mint authority account
    #[account(
        seeds = ["mint-authority".as_bytes()], // Seed for the mint authority
        bump, // Bump seed
    )]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>, // Payer account
    pub rent: Sysvar<'info, Rent>, // Rent sysvar
    pub token_program: Program<'info, Token>, // Token program account
    pub system_program: Program<'info, System>, // System program account
}

// Define account structure for the airdrop
#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(
        mut,
        seeds = ["token-mint".as_bytes()], // Seed for the token mint account
        bump // Bump seed
    )]
    pub token_mint: Account<'info, Mint>, // Token mint account
    // Mint authority account
    #[account(
        seeds = ["mint-authority".as_bytes()], // Seed for the mint authority
        bump // Bump seed
    )]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>, // User account (signer)
    #[account(
        init,
        token::mint = token_mint, // Associated token mint
        token::authority = user, // Token authority
        payer = user // Payer of the transaction
    )]
    pub user_token_account: Account<'info, TokenAccount>, // User's token account
    pub rent: Sysvar<'info, Rent>, // Rent sysvar
    pub token_program: Program<'info, Token>, // Token program account
    pub system_program: Program<'info, System>, // System program account
}

// Implementation for Airdrop
impl<'info> Airdrop<'info> {
    // Create a CPI context for the MintTo instruction
    pub fn mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        // Get the CPI program account
        let cpi_program = self.token_program.to_account_info();
        // Create CPI accounts for MintTo instruction
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };

        // Create and return a new CPI context
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
