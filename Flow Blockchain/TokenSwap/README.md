
# Fungible Token Project

Welcome to the Fungible Token Project repository, a Flow blockchain project that implements a custom Fungible Token contract and various transactions and scripts for token management and interactions. This project is divided into several parts, each addressing specific aspects of token management and interactions.

## Part 1: The FlowToken Contract

### Overview

The `FlowToken` contract serves as the cornerstone of this project, offering essential features for managing fungible tokens. Key functionalities include:

-   **Minting Control:** The contract owner can mint new tokens.
-   **Vault Resource:** Balances are stored within the `Vault` resource.
-   **Token Management:** Various transactions and scripts enable token management.

### Code Walk-Through

One notable feature is the `deposit` function within the `Vault` resource. It ensures secure token transfer and prevents double counting. By zeroing out the incoming vault's balance before destroying it, it guarantees a smooth transition.

## Part 2: Transactions and Scripts

### Transactions

1.  **MINT:** Mint tokens to a specified recipient.
2.  **SETUP:** Properly sets up a vault within a user's account storage.
3.  **TRANSFER:** Allows users to transfer tokens to different addresses.

### Scripts

1.  **READ BALANCE:** Retrieves the balance of a user's vault.
2.  **SETUP CORRECTLY?:** Checks if a user's vault is set up correctly.
3.  **TOTAL SUPPLY:** Reports the total supply of tokens in circulation.

## Part 3: Transactions and Scripts Modification

### Transactions

-   **SETUP:** Identifies and rectifies poorly set up vaults.
-   **READ BALANCE:** Works with poorly set up vaults and temporarily fixes them to ensure balance retrieval.

## Part 4: Contract Modification

### Overview

The `Admin` role is granted the ability to withdraw tokens from a user's vault and deposit equivalent $FLOW tokens.

### Transactions

-   **Admin Withdraw and Deposit:** Allows the Admin to withdraw tokens and deposit $FLOW tokens.

## Part 5: Additional Scripts

### Scripts

1.  **Balance Summary:** Returns the balance of the user's $FLOW vault and custom vault.
2.  **Vault Overview:** Provides a structured summary of all official Fungible Token vaults in the user's account storage.

## Part 6: Swap Contract

### Overview

The `Swap` contract empowers users to deposit $FLOW tokens and receive custom tokens in return. The received amount is based on the time since their last swap.

### Swapping Functionality

Two methods ensure user identity and authenticity:

1.  Custom identity resource to represent identity.
2.  Reference to the user's $FLOW vault to verify authenticity.

## Conclusion

The Flow Token project showcases the implementation of a custom Fungible Token contract and its diverse functionalities. This repository includes contracts, transactions, and scripts for managing tokens, setting up vaults, transferring tokens, and swapping tokens. By organizing code and functionalities into distinct parts, the project becomes more comprehensible and manageable.

For comprehensive usage and instructions, please refer to the individual code files and comments within the repository.
