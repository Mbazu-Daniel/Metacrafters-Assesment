// Import necessary contracts and libraries
import FungibleToken from 0x01
import BazuToken from 0x01

// Define the main function that verifies a user's vault setup
pub fun main(account: Address) {

    // Borrow the BazuToken Vault with FungibleToken.Balance interface from the user's account
    let publicVault = getAccount(account)
        .getCapability(/public/Vault)
        .borrow<&BazuToken.Vault{FungibleToken.Balance}>()
        ?? panic("Vault not found or setup is incomplete: Please make sure to set up the Vault")

    // Log a success message if the vault setup is verified successfully
    log("Vault setup verified successfully")
}
