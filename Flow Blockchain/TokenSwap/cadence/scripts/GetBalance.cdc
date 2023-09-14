// Import necessary contracts and libraries
import FungibleToken from 0x01
import BazuToken from 0x01

// Define the main function that checks and creates a user's vault
pub fun main(account: Address) {

    // Attempt to borrow the BazuToken Vault with required interfaces
    let publicVault: &BazuToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, BazuToken.CollectionPublic}? =
        getAccount(account).getCapability(/public/Vault)
            .borrow<&BazuToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, BazuToken.CollectionPublic}>()

    if (publicVault == nil) {
        // Create an empty vault if it doesn't exist
        let newVault <- BazuToken.createEmptyVault()
        getAuthAccount(account).save(<-newVault, to: /storage/VaultStorage)
        getAuthAccount(account).link<&BazuToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, BazuToken.CollectionPublic}>(
            /public/Vault,
            target: /storage/VaultStorage
        )
        log("Vault setup completed: An empty vault has been created")
        
        // Retrieve the newly created vault's balance and log it
        let retrievedVault: &BazuToken.Vault{FungibleToken.Balance}? =
            getAccount(account).getCapability(/public/Vault)
                .borrow<&BazuToken.Vault{FungibleToken.Balance}>()
        log("Vault Balance: \(retrievedVault?.balance ?? 0.0)")
    } else {
        // Log a message indicating that the vault already exists
        log("Vault setup completed: The vault already exists and is properly linked")
        
        // Check if this vault is associated with BazuToken
        let checkVault: &BazuToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, BazuToken.CollectionPublic} =
            getAccount(account).getCapability(/public/Vault)
                .borrow<&BazuToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, BazuToken.CollectionPublic}>()
                ?? panic("Vault capability not found")

        if BazuToken.vaults.contains(checkVault.uuid) {
            // If the vault is associated with BazuToken, log its balance
            log("Vault Balance: \(publicVault?.balance ?? 0.0)")
            log("This is a BazuToken vault")
        } else {
            // If the vault is not associated with BazuToken, log a message
            log("This is not a BazuToken vault")
        }
    }
}
