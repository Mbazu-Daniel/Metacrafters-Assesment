
# CryptoPals Contract

This is a smart contract written in Cadence for the CryptoPals application. The contract extends the `NonFungibleToken` contract and provides functionality for managing and interacting with non-fungible tokens (NFTs).

## Contract Structure

The `CryptoPals` contract consists of the following main components:

-   `NFT` Resource: Represents a non-fungible token with specific attributes such as `name`, `favouriteFood`, and `luckyNumber`.
-   `Collection` Resource: Implements the `NonFungibleToken.Provider`, `NonFungibleToken.Receiver`, and `NonFungibleToken.CollectionPublic` interfaces to manage collections of NFTs. It allows users to deposit and withdraw NFTs, get the IDs of owned NFTs, and borrow NFTs for temporary usage.
-   `Minter` Resource: Provides functions to create new NFTs and minters.

## Events

The following events are emitted by the `CryptoPals` contract:

-   `ContractInitialized`: Indicates that the contract has been initialized.
-   `Withdraw`: Indicates a successful withdrawal of an NFT from a collection. Includes the ID of the withdrawn NFT and the address of the sender.
-   `Deposit`: Indicates a successful deposit of an NFT into a collection. Includes the ID of the deposited NFT and the address of the receiver.

## Functionality

The `CryptoPals` contract provides the following core functionality:

-   Creation of empty collections: Users can create an empty collection by calling the `createEmptyCollection` function.
-   Creation of NFTs: The `Minter` resource allows users to create new NFTs with specified attributes using the `createNFT` function.
-   Deposit and withdrawal: Users can deposit and withdraw NFTs to/from their collections using the `deposit` and `withdraw` functions, respectively.
-   Access to owned NFTs: The `Collection` resource provides functions to retrieve the IDs of owned NFTs (`getIDs`) and borrow NFTs for temporary usage (`borrowNFT` and `borrowAuthNFT`).

## Initialization

During contract initialization, the following actions are performed:

-   The `totalSupply` variable is initialized to 0.
-   An instance of the `Minter` resource is created and stored in the `/storage/Minter` path.
-   The `ContractInitialized` event is emitted.

Please note that this is a high-level overview of the contract's functionality. For more details, refer to the code implementation provided above.

##   Transactions

The following transactions demonstrate the usage of the `CryptoPals` contract:

### Transaction 1: Initialize Contract and Link Collection

`import CryptoPals from "../contracts/CryptoPals.cdc";
transaction() {
  prepare(signer: AuthAccount) {
    // Store a `CryptoPals.Collection` in our account storage.
    signer.save(<- CryptoPals.createEmptyCollection(), to: /storage/CryptoPals)
   // Link it to the public.
    signer.link<&CryptoPals.Collection>(/public/CryptoPals, target: /storage/CryptoPals)
  }}`

This transaction initializes the contract by creating an empty collection (`CryptoPals.Collection) and storing it in the account storage. It also links the collection to the public interface, allowing other accounts to interact with it.

### Transaction 2: Mint and Deposit NFT
` import CryptoPals from "../contracts/CryptoPals.cdc";
import NonFungibleToken from "../contracts/NonFungibleToken.cdc"

transaction(recipient: Address) {
  prepare(signer: AuthAccount) {
    // Get a reference to the `Minter`
    let minter = signer.borrow<&CryptoPals.Minter>(from: /storage/Minter)
                    ?? panic("This signer is not the one who deployed the contract.")

    // Get a reference to the `recipient`s public Collection
    let recipientsCollection = getAccount(recipient).getCapability(/public/CryptoPals)
              .borrow<&CryptoPals.Collection{NonFungibleToken.CollectionPublic}>()
              ?? panic("The address does not have a Collection.")

    // Mint the NFT using the reference to the `Minter`
    let nft <- minter.createNFT(name: "CryptoPals", favouriteFood: "Akara", luckyNumber: 17)

    // Deposit the NFT in the recipient's Collection
    recipientsCollection.deposit(token: <- nft) }}```

This transaction demonstrates the minting and depositing of an NFT into a recipient's collection. It assumes that the `signer` is the one who deployed the contract and has access to the `Minter` resource. The recipient's public collection is obtained, and an NFT is created using the `Minter` reference. The newly minted NFT is then deposited into the recipient's collection.

Please note that these transactions are provided as examples and should be adapted to your specific use case.


## Scripts

The following scripts provide examples of interacting with the `CryptoPals` contract:

### Script 1: Get Collection Size

cadenceCopy code

`import CryptoPals from "../contracts/CryptoPals.cdc";
import NonFungibleToken from "../contracts/NonFungibleToken.cdc"

pub fun main(address: Address): Int {
  let publicCollection = getAccount(address).getCapability(/public/CryptoPals)
              .borrow<&CryptoPals.Collection>()
              ?? panic("The address does not have a Collection.")

  let value = publicCollection.getIDs().length
  log(value)
  return value
}` 

This script retrieves the public collection of an address and returns the size of the collection. It uses the `getIDs` function of the collection to obtain an array of NFT IDs and calculates its length.

### Script 2: Get NFT Details

```import CryptoPals from "../contracts/CryptoPals.cdc";
import NonFungibleToken from "../contracts/NonFungibleToken.cdc"

pub fun main(address: Address, id: UInt64): &CryptoPals.NFT {
  let publicCollection = getAccount(address).getCapability(/public/CryptoPals)
              .borrow<&CryptoPals.Collection>()
              ?? panic("The address does not have a Collection.")

  let nftRef = publicCollection.borrowAuthNFT(id: id)

  log(nftRef.name)
  log(nftRef.favouriteFood)
  log(nftRef.luckyNumber)

  return nftRef
}``` 

This script retrieves an NFT from a public collection using its ID. It accesses the collection of the specified address and borrows the authenticated NFT using the `borrowAuthNFT` function. The details of the NFT, including `name`, `favouriteFood`, and `luckyNumber`, are logged.

Please note that these scripts are provided as examples and should be adapted to your specific use case.
