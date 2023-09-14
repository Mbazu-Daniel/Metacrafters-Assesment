// Import necessary contracts and libraries
import FungibleToken from 0x01
import BazuToken from 0x01

// Define a transaction to manage a user's BazuToken vault
transaction() {

    // Declare a reference to the user's BazuToken Vault with specific interfaces
    let userVault: &BazuToken.Vault{FungibleToken.Balance, FungibleToken.Provider, FungibleToken.Receiver, BazuToken.CollectionPublic}?
    
    // Declare the AuthAccount
    let account: AuthAccount

    // Prepare the transaction
    prepare(acct: AuthAccount) {

        // Borrow the user's BazuToken Vault capability with specific interfaces from public
        self.userVault = acct.getCapability(/public/Vault)
            .borrow<&BazuToken.Vault{FungibleToken.Balance, FungibleToken.Provider, FungibleToken.Receiver, BazuToken.CollectionPublic}>()

        // Store the AuthAccount reference
        self.account = acct
    }

    // Execute the transaction
    execute {
        if self.userVault == nil {
       
            // Create an empty BazuToken vault if it doesn't exist
            let emptyVault <- BazuToken.createEmptyVault()
            self.account.save(<-emptyVault, to: /storage/VaultStorage)
            self.account.link<&BazuToken.Vault{FungibleToken.Balance, FungibleToken.Provider, FungibleToken.Receiver, BazuToken.CollectionPublic}>(/public/Vault, target: /storage/VaultStorage)
            log("BazuToken vault setup completed: An empty BazuToken vault has been created and linked")
        } else {
            // Log a message indicating that the BazuToken vault already exists
            log("BazuToken vault setup completed: The BazuToken vault already exists and is properly linked")
        }
    }
}
