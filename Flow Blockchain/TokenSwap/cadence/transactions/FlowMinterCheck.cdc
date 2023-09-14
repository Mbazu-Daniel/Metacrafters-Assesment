// Import necessary contracts and libraries
import FungibleToken from 0x01
import FlowToken from 0x01

// Define a transaction to interact with the FlowToken Minter
transaction() {

  // Declare a reference to the FlowToken Minter
  let flowMinter: &FlowToken.Minter

  // Prepare the transaction
  prepare(acct: AuthAccount) {

    // Borrow the FlowToken Minter resource from storage
    self.flowMinter = acct.borrow<&FlowToken.Minter>(from: /storage/FlowMinter)
        ?? panic("FlowToken Minter is not present")
    
    // Log a message indicating the presence of FlowToken Minter
    log("FlowToken Minter is present")
  }

  // Execute the transaction
  execute {
    // This transaction is empty, no action is taken here
  }
}
