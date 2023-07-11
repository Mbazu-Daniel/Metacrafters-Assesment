# BazRiotNFT Contract

This contract is an ERC721A compliant contract that allows the minting and management of BazRiotNFTs. The contract extends the ERC721A contract and provides additional functionality specific to BazRiotNFT.

## Contract Details

-   Contract name: BazRiotNFT
-   Contract symbol: BRNFT
-   SPDX-License-Identifier: MIT

## Prerequisites

Before using this contract, make sure you have the following:

-   Solidity version 0.8.9 or higher.
-   The ERC721A contract imported and available in the project.

## Contract Functions

### Constructor

The constructor function initializes the BazRiotNFT contract by setting the contract owner as the deployer of the contract.

### Modifier

-   `onlyOwner`: This modifier ensures that only the owner of the contract can perform certain actions.

### External Functions

-   `mint(uint256 quantity)`: This function allows the owner of the contract to mint a specified quantity of BazRiotNFT tokens. The `quantity` parameter determines the number of tokens to be minted. The function checks if the total supply plus the requested quantity does not exceed the maximum quantity allowed (5 in this case) before minting the tokens.
    
-   `promptDescription()`: This function returns the URL for the prompt description associated with the BazRiotNFT. The prompt description is a string that provides information about the image of a single woman in a riot scene in Nigeria.
    

### Internal Functions

-   `_baseURI()`: This internal function is overridden from the ERC721A contract and returns the base URL for the BazRiotNFTs. The base URL is used as a prefix for generating the token URI for each NFT.

## Contract Variables

-   `owner`: This variable stores the address of the contract owner.
    
-   `maxQuantity`: This variable defines the maximum number of tokens that can be minted (5 in this case).
    
-   `baseUrl`: This variable stores the base URL for the BazRiotNFTs. It is used as a prefix to generate the complete token URI for each NFT.
    
-   `PROMPT_NAME`: This variable stores the prompt description for the BazRiotNFTs. It is a string representing the image of a single woman in a riot scene in Nigeria.
    

## License

This contract is licensed under the MIT License. Please refer to the SPDX-License-Identifier comment at the beginning of the contract for more details.
