// Import the FungibleToken library from 0x01
import FungibleToken from 0x01

// Main function to retrieve the balances of user's FungibleToken Vaults
pub fun main(user: Address): {UInt64: UFix64} {

    // Get the user's AuthAccount
    let authAccount = getAuthAccount(user)

    // Declare a dictionary to store vault UUIDs and balances
    var vaults: {UInt64: UFix64} = {}

    // Iterate through stored resources in the user's account
    authAccount.forEachStored(fun(path: StoragePath, type: Type): Bool {

        // Check if the stored resource is a subtype of FungibleToken.Vault
        if type.isSubtype(of: Type<@FungibleToken.Vault>()) {

            // Borrow the FungibleToken.Vault reference from the path
            let vaultRef = authAccount.borrow<&FungibleToken.Vault>(from: path)!

            // Store the vault's UUID and balance in the dictionary
            vaults[vaultRef.uuid] = vaultRef.balance
        }

        return true
    })

    // Return the dictionary containing vault UUIDs and balances
    return vaults
}
