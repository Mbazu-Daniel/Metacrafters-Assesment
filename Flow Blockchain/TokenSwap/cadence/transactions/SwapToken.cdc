// Import the SwapToken contract from 0x01
import SwapToken from 0x01

// Define a transaction to swap tokens using the SwapToken contract
transaction(amount: UFix64) {

    // Declare the AuthAccount of the transaction's signer
    let signer: AuthAccount

    // Prepare the transaction
    prepare(acct: AuthAccount) {
        // Store the signer's AuthAccount reference
        self.signer = acct
    }

    // Execute the transaction
    execute {
        // Call the `swapTokens` function from the SwapToken contract
        SwapToken.swapTokens(signer: self.signer, swapAmount: amount)
        
        // Log a message to confirm the swap
        log("Token swap completed")
    }
}
