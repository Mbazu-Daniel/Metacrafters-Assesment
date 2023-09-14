// Import necessary contracts and libraries
import FungibleToken from 0x01
import BazuToken from 0x01

// Define a transaction to transfer BazuTokens between accounts
transaction(receiverAccount: Address, amount: UFix64) {

    // Declare references to the signer's and receiver's BazuToken Vaults
    let signerVault: &BazuToken.Vault
    let receiverVault: &BazuToken.Vault{FungibleToken.Receiver}

    // Prepare the transaction
    prepare(acct: AuthAccount) {
        // Borrow the signer's BazuToken Vault from storage
        self.signerVault = acct.borrow<&BazuToken.Vault>(from: /storage/VaultStorage)
            ?? panic("Vault not found in senderAccount")

        // Retrieve the receiver's BazuToken Vault capability from public
        self.receiverVault = getAccount(receiverAccount)
            .getCapability(/public/Vault)
            .borrow<&BazuToken.Vault{FungibleToken.Receiver}>()
            ?? panic("Vault not found in receiverAccount")
    }

    // Execute the transaction
    execute {
        // Transfer tokens from signer's Vault to receiver's Vault and log the action
        self.receiverVault.deposit(from: <-self.signerVault.withdraw(amount: amount))
        log("BazuTokens transferred successfully")
    }
}
