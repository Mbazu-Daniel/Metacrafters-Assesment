// Import the FungibleToken library from 0x01
import FungibleToken from 0x01

// Define a public contract called BazuToken that extends FungibleToken
pub contract BazuToken: FungibleToken {

    // Declare a variable to store the total supply of tokens
    pub var totalSupply: UFix64

    // Declare an array to store the UUIDs of vaults
    pub var vaults: [UInt64]

    // Declare public events for token-related actions
    pub event TokensInitialized(initialSupply: UFix64)
    pub event TokensWithdrawn(amount: UFix64, from: Address?)
    pub event TokensDeposited(amount: UFix64, to: Address?)

    // Declare a resource interface for interacting with token balances
    pub resource interface CollectionPublic {
        pub var balance: UFix64
        pub fun deposit(from: @FungibleToken.Vault)
        pub fun withdraw(amount: UFix64): @FungibleToken.Vault
        access(contract) fun adminWithdraw(amount: UFix64): @FungibleToken.Vault
    }

    // Define the Vault resource that implements various FungibleToken interfaces
    pub resource Vault: FungibleToken.Provider, FungibleToken.Receiver, FungibleToken.Balance, CollectionPublic {
        pub var balance: UFix64

        // Initialize the Vault with a balance
        init(balance: UFix64) {
            self.balance = balance
        }

        // Withdraw tokens from the Vault
        pub fun withdraw(amount: UFix64): @FungibleToken.Vault {
            self.balance = self.balance - amount
            emit TokensWithdrawn(amount: amount, from: self.owner?.address)
            return <-create Vault(balance: amount)
        }

        // Deposit tokens into the Vault
        pub fun deposit(from: @FungibleToken.Vault) {
            let vault <- from as! @BazuToken.Vault
            emit TokensDeposited(amount: vault.balance, to: self.owner?.address)
            self.balance = self.balance + vault.balance
            vault.balance = 0.0
            destroy vault
        }

        // Admin function to withdraw tokens from the Vault
        access(contract) fun adminWithdraw(amount: UFix64): @FungibleToken.Vault {
            self.balance = self.balance - amount
            return <-create Vault(balance: amount)
        }

        // Destructor to update the total supply when a Vault is destroyed
        destroy() {
            BazuToken.totalSupply = BazuToken.totalSupply - self.balance
        }
    }

    // Define a resource called Admin
    pub resource Admin {
        // Admin function to get tokens from a sender's Vault
        pub fun adminGetCoin(senderVault: &Vault{CollectionPublic}, amount: UFix64): @FungibleToken.Vault {
            return <-senderVault.adminWithdraw(amount: amount)
        }
    }

    // Define a resource called Minter
    pub resource Minter {
        // Minter function to create and mint tokens
        pub fun mintToken(amount: UFix64): @FungibleToken.Vault {
            BazuToken.totalSupply = BazuToken.totalSupply + amount
            return <-create Vault(balance: amount)
        }
    }

    // Initialize the BazuToken contract
    init() {
        // Initialize the total supply to 0
        self.totalSupply = 0.0

        // Create and save a Minter resource
        self.account.save(<-create Minter(), to: /storage/MinterStorage)
        self.account.link<&BazuToken.Minter>(/public/Minter, target: /storage/MinterStorage)

        // Create and save an Admin resource
        self.account.save(<-create Admin(), to: /storage/AdminStorage)

        // Initialize the vaults array
        self.vaults = []

        // Emit an event to indicate token initialization
        emit TokensInitialized(initialSupply: self.totalSupply)
    }

    // Function to create an empty Vault and return it
    pub fun createEmptyVault(): @FungibleToken.Vault {
        let instance <- create Vault(balance: 0.0)
        self.vaults.append(instance.uuid)
        return <-instance
    }
}
