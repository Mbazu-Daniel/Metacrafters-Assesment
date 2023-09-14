// Import necessary contracts and libraries
import FungibleToken from 0x01
import FlowToken from 0x01

// Function to retrieve the balance of a user's FlowToken Vault
pub fun getFlowVaultBalance(account: Address): UFix64? {

    // Attempt to borrow the FlowToken Vault with the FungibleToken.Balance interface
    let publicFlowVault: &FlowToken.Vault{FungibleToken.Balance}?
        = getAccount(account)
            .getCapability(/public/FlowVault)
            .borrow<&FlowToken.Vault{FungibleToken.Balance}>()

    if let balance = publicFlowVault?.balance {
        // Return the balance if it exists
        return balance
    } else {
        // Return an error message if the vault doesn't exist or borrowing failed
        return panic("Flow vault does not exist or borrowing failed: Please ensure that the FlowVault is properly set up")
    }
}

// Main function to retrieve the FlowToken Vault balance
pub fun main(_account: Address): UFix64? {

    // Call the getFlowVaultBalance function and return the result
    return getFlowVaultBalance(account: _account)
}
