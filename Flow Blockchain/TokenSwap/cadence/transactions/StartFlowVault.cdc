// Import necessary contracts and libraries
import FungibleToken from 0x01
import FlowToken from 0x01

// Define a transaction to manage FlowToken vault
transaction() {

    // Declare a reference to the FlowToken Vault
    let flowTokenVault: &FlowToken.Vault?
    
    // Declare the AuthAccount
    let account: AuthAccount

    // Prepare the transaction
    prepare(acct: AuthAccount) {

        // Borrow the FlowToken Vault capability from public
        self.flowTokenVault = acct.getCapability(/public/FlowVault)
            .borrow<&FlowToken.Vault>()

        // Store the AuthAccount reference
        self.account = acct
    }

    // Execute the transaction
    execute {
        if self.flowTokenVault == nil {
         
            // Create an empty FlowToken vault if it doesn't exist
            let newVault <- FlowToken.createEmptyVault()
            self.account.save(<-newVault, to: /storage/FlowVault)
            self.account.link<&FlowToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, FungibleToken.Provider}>(/public/FlowVault, target: /storage/FlowVault)
            log("FlowToken vault setup completed: An empty FlowToken vault has been created and linked")
        } else {
            // Log a message indicating that the FlowToken vault already exists
            log("FlowToken vault setup completed: The FlowToken vault already exists and is properly linked")
        }
    }
}
