
# Counter Program 

This README provides an explanation of the Solana blockchain program defined in the Rust code using the Anchor framework.

## Overview

This Solana program manages a simple counter on the Solana blockchain. It allows users to create a counter account, increment its value, and decrement its value. The program enforces proper authorization and account structures for secure and reliable operation.

## Program Structure

The code consists of several components:

1.  **Program Declaration**: The program is identified by a unique ID used for interactions on the Solana blockchain.
    
2.  **Program Module**: The program module `counter_program` contains three functions: `create`, `increment`, and `decrement`. Each function serves a specific purpose related to the counter.
    
3.  **Account Structures**: There are three account structures: `Create`, `Increment`, and `Decrement`, each representing the accounts needed for their respective functions.
    
4.  **Counter Account**: The `Counter` account structure holds the counter's state, including the authority's public key and the current count value.
    

## Functions

### `create`

-   **Purpose**: This function creates a new counter account.
-   **Actions**:
    -   Sets the authority of the counter account to the provided authority's key.
    -   Initializes the count to 0.
-   **Usage**: This function is used to initialize a new counter account before it can be incremented or decremented.

### `increment`

-   **Purpose**: This function increments the count in the counter account.
-   **Actions**:
    -   Increases the count by 1.
-   **Usage**: Use this function to increment the counter's value.

### `decrement`

-   **Purpose**: This function decrements the count in the counter account.
-   **Actions**:
    -   Decreases the count by 1.
-   **Usage**: Use this function to decrement the counter's value.

## Account Structures

### `Create`

-   **Purpose**: Represents the accounts required for the `create` function.
-   **Components**:
    -   `counter`: Initialized counter account.
    -   `authority`: Mutable authority account.
    -   `system_program`: Solana system program account.

### `Increment`

-   **Purpose**: Represents the accounts required for the `increment` function.
-   **Components**:
    -   `counter`: Mutable counter account with the authority.
    -   `authority`: Authority signer account.

### `Decrement`

-   **Purpose**: Represents the accounts required for the `decrement` function.
-   **Components**:
    -   `counter`: Mutable counter account with the authority.
    -   `authority`: Authority signer account.

## Getting Started

To use this Solana program, follow these steps:

1.  Deploy the program on the Solana blockchain.
    
2.  Interact with the program using the Solana client, sending transactions to call the `create`, `increment`, and `decrement` functions.
    
3.  Ensure you have the required authority to perform these operations.
    

This program serves as a basic example of creating and interacting with a Solana smart contract. You can build more complex applications by extending its functionality as needed.

For more details on how to deploy and interact with Solana programs using Anchor, refer to the official [Anchor documentation](https://project-serum.github.io/anchor/).