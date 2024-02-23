
# Solana Escrow Program

This Solana Anchor program implements a simple escrow system on the Solana blockchain. It allows users to create escrow accounts and withdraw funds from them after a specified timelock period.

## Features

-   **Create Escrow**: Users can create escrow accounts by specifying the beneficiary, amount, and timelock duration.
-   **Withdraw Funds**: Beneficiaries can withdraw funds from the escrow account after the timelock period has expired.
-   **Security**: The program ensures that only the owner can withdraw funds and that withdrawals occur after the timelock period has elapsed.

## Usage

To use this escrow program, follow these steps:

1.  **Build the Program**: Build the Solana program using Anchor. Make sure you have Anchor installed and configured.
    
    bash
    

-   `anchor build` 
    
-   **Deploy the Program**: Deploy the built program to the Solana blockchain. You can deploy it to a local network for testing or to a live network.
    
    bash
    

1.  `anchor deploy` 
    
2.  **Interact with the Program**: Once deployed, interact with the program using client applications or scripts. You can create escrow accounts and withdraw funds using the provided program instructions.
    

## Testing

You can test the escrow program using the provided test scripts. The test scripts are written in TypeScript and use Mocha for testing. Before running the tests, make sure to set up your environment and ensure that the program is deployed to a local network.

To run the tests, execute the following command:

bash

`mocha path/to/testfile.ts`
