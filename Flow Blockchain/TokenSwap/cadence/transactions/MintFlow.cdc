// Import necessary contracts and libraries
import FungibleToken from 0x01
import FlowToken from 0x01

// Define a transaction to mint FlowTokens
transaction(amountToMint: UFix64) {
  
    // Declare a reference to the FlowToken Minter
    let minter: &FlowToken.Minter

    // Declare a reference to the signer's AuthAccount
    let signer: AuthAccount

    // Prepare the transaction
    prepare(signerRef: AuthAccount) {
  
        // Store the signer's AuthAccount reference
        self.signer = signerRef
     
        // Borrow the FlowToken Minter resource from storage
        self.minter = self.signer.borrow<&FlowToken.Minter>(from: /storage/FlowMinter)
            ?? panic("Minter resource not found")
    }

    // Execute the transaction
    execute {

        // Mint new FlowTokens with the provided amount
        let newTokens <- self.minter.mintTokens(amount: amountToMint)

        // Save the newly minted tokens to the signer's FlowVault
        self.signer.save(<-newTokens, to: /storage/FlowVault)

        // Log a success message indicating the minted amount
        log("Minted \(amountToMint) FlowTokens successfully")
    }
}
