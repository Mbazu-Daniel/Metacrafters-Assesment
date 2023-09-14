// Import necessary contracts and libraries
import FungibleToken from 0x01
import FlowToken from 0x01
import BazuToken from 0x01

// Define the SwapToken contract
pub contract SwapToken {

    // Declare a variable to track the last swap timestamp
    pub var lastSwapTimestamp: UFix64

    // Declare a map to store the last swap timestamps for users
    pub var userLastSwapTimestamps: {Address: UFix64}

    // Function to allow users to swap tokens
    pub fun swapTokens(signer: AuthAccount, swapAmount: UFix64) {
        
        // Borrow the BazuToken Vault from the signer's storage
        let bazuTokenVault = signer.borrow<&BazuToken.Vault>(from: /storage/VaultStorage)
            ?? panic("Borrowing BazuToken Vault failed: Vault not found")

        // Borrow the FlowToken Vault from the signer's storage
        let flowVault = signer.borrow<&FlowToken.Vault>(from: /storage/FlowVault)
            ?? panic("Borrowing FlowToken Vault failed: Vault not found")

        // Borrow the Minter capability from BazuToken
        let minterRef = signer.getCapability<&BazuToken.Minter>(/public/Minter).borrow()
            ?? panic("Borrowing BazuToken Minter reference failed: Minter not found")

        // Borrow the FlowToken Vault capability with FungibleToken interfaces
        let autherVault = signer.getCapability<&FlowToken.Vault{FungibleToken.Balance, FungibleToken.Receiver, FungibleToken.Provider}>(/public/FlowVault).borrow()
            ?? panic("Borrowing FlowToken Vault reference failed: Vault not found")

        // Withdraw tokens from FlowToken Vault and deposit them into autherVault
        let withdrawnAmount <- flowVault.withdraw(amount: swapAmount)
        autherVault.deposit(from: <-withdrawnAmount)

        // Get the user's address and retrieve their last swap timestamp
        let userAddress = signer.address
        self.lastSwapTimestamp = self.userLastSwapTimestamps[userAddress] ?? 1.0
        let currentTime = getCurrentBlock().timestamp

        // Calculate the time since the last swap and the minted token amount
        let timeSinceLastSwap = currentTime - self.lastSwapTimestamp
        let mintedAmount = 2.0 * UFix64(timeSinceLastSwap)

        // Mint new BazuTokens and deposit them into the user's BazuToken Vault
        let newBazuTokenVault <- minterRef.mintToken(amount: mintedAmount)
        bazuTokenVault.deposit(from: <-newBazuTokenVault)

        // Update the user's last swap timestamp
        self.userLastSwapTimestamps[userAddress] = timeSinceLastSwap
    }

    // Initialize the SwapToken contract
    init() {
        // Initialize the lastSwapTimestamp to 1.0 and set a default timestamp for user 0x01
        self.lastSwapTimestamp = 1.0
        self.userLastSwapTimestamps = {0x01: 1.0}
    }
}
