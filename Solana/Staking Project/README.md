
# Solana Staking Program

This is a Rust program developed using the Anchor framework to run on the Solana blockchain. It implements a staking mechanism with token transfers, allowing users to initialize token mints, airdrop tokens, stake tokens, and unstake tokens.

## Table of Contents


## 1. Introduction
The Solana Staking Program is designed to provide staking functionality on the Solana blockchain. Users can stake and unstake tokens, and the program ensures proper accounting of token balances.

## 2. Program Overview

### Program Structure

-   The program is organized into a main module called `solana_stake`, which contains the staking logic.
-   It uses the Anchor framework for Solana program development.
-   Token-related functionality is implemented using the `anchor_spl` library, which provides support for interacting with tokens.

### Key Functions

-   `init_mint`: Initializes a new token mint with a specified number of decimals.
-   `airdrop`: Allows users to airdrop tokens to an account by using seeds and the `mint_to` instruction.
-   `stakeToken`: Enables users to stake tokens from their account to a vault, updating the stake amount.
-   `unstakeToken`: Allows users to unstake tokens from the vault to their account, updating the stake amount.

## 3. Usage

To use the Solana Staking Program, you can follow these steps:

1.  **Initialize Mint**: Call the `init_mint` function to create a new token mint with the desired properties, such as authority and decimals.
    
2.  **Airdrop Tokens**: Use the `airdrop` function to airdrop tokens to a specific account. Provide the amount and necessary authorization.
    
3.  **Stake Tokens**: Stake tokens by calling the `stakeToken` function, specifying the amount to stake. The program checks the account's token balance before staking.
    
4.  **Unstake Tokens**: Unstake tokens with the `unstakeToken` function. Ensure that the account has enough staked tokens. The program transfers tokens from the vault to the account.
    

## 4. Error Handling
The program defines custom error codes to handle various error scenarios:

-   `AccountInsufficientTokenBalance`: Raised when there are not enough tokens in an account to perform an operation.
-   `UnstakingTooManyTokens`: Raised when attempting to unstake more tokens than are currently staked.

## 5. Account Structures

The program uses several account structures with specific requirements:

-   `InitializeMint`: Structure for initializing a token mint.
-   `Airdrop`: Structure for airdropping tokens to an account.
-   `Staking`: Structure for staking tokens and managing stake-related accounts.
-   `Stake`: Structure representing a staked amount.

## 6. Context Creation

The program provides helper functions to create contexts for invoking token-related instructions:

-   `mint_to_context`: Creates a context for the `mint_to` instruction.
-   `transfer_to_vault_context`: Creates a context for transferring tokens to the vault.
-   `transfer_to_account_context`: Creates a context for transferring tokens from the vault to an account.

For more details, refer to the code and function comments.

**Note**: This readme provides an overview of the Solana Staking Program. Be sure to review the code and function comments for more in-depth details and usage instructions.