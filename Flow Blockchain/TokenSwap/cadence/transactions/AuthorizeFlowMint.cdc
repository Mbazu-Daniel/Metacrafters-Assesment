// Import necessary contracts and libraries
import FungibleToken from 0x01
import FlowToken from 0x01

// Define a transaction to create a new FlowToken Minter
transaction(_allowedAmount: UFix64){
   
    // Declare a reference to the FlowToken Administrator
    let admin: &FlowToken.Administrator

    // Declare a reference to the signer's AuthAccount
    let signer: AuthAccount

    // Prepare the transaction
    prepare(signerRef: AuthAccount) {
        // Store the signer's AuthAccount reference
        self.signer = signerRef
     
        // Borrow the FlowToken Administrator resource from storage
        self.admin = self.signer.borrow<&FlowToken.Administrator>(from: /storage/newflowTokenAdmin)
            ?? panic("You are not the admin")
    }

    // Execute the transaction
    execute {
        // Create a new FlowToken Minter with the provided allowed amount
        let newMinter <- self.admin.createNewMinter(allowedAmount: _allowedAmount)

        // Save the new Minter resource to storage
        self.signer.save(<-newMinter, to: /storage/FlowMinter)

        // Log a success message
        log("Flow minter created successfully")
    }
}
