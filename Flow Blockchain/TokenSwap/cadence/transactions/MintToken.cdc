// Import necessary contracts and libraries
import FungibleToken from 0x01
import BazuToken from 0x01

// Define a transaction to mint and deposit BazuTokens to a receiver's Vault
transaction(receiver: Address, amount: UFix64) {

    // Prepare the transaction
    prepare(signer: AuthAccount) {

        // Borrow the BazuToken Minter resource from storage
        let minter = signer.borrow<&BazuToken.Minter>(from: /storage/MinterStorage)
            ?? panic("You are not the BazuToken minter")
        
        // Retrieve the receiver's BazuToken Vault capability from public
        let receiverVault = getAccount(receiver)
            .getCapability<&BazuToken.Vault{FungibleToken.Receiver}>(/public/Vault)
            .borrow()
            ?? panic("Error: Check your BazuToken Vault status")
        
        // Mint new BazuTokens with the provided amount
        let mintedTokens <- minter.mintToken(amount: amount)

        // Deposit the minted tokens into the receiver's Vault
        receiverVault.deposit(from: <-mintedTokens)
    }

    // Execute the transaction
    execute {
        // Log a success message with the minted amount
        log("Minted and deposited BazuTokens successfully")
        log(amount.toString().concat(" Tokens minted and deposited"))
    }
}
