
## DegenToken Smart Contract

This is a Solidity smart contract for the DegenToken, an ERC20 token deployed on the Avalanche network. The contract extends the functionality of the OpenZeppelin `ERC20` and `Ownable` contracts, providing features such as minting, transferring, redeeming, burning, and checking token balances.

### Contract Details

-   Contract Name: DegenToken
-   Symbol: DGN
-   Decimals: 18
-   Initial Supply: 0

### Functions

#### `constructor()`

The constructor function initializes the token with the name "Degen" and the symbol "DGN". It is executed only once during contract deployment.

#### `mint(address to, uint256 amount)`

The `mint` function allows the owner of the contract to create and distribute new tokens. It takes two parameters: the recipient address (`to`) and the amount of tokens to be minted (`amount`). Only the contract owner is authorized to call this function.

#### `transfer(address to, uint256 amount)`

The `transfer` function allows token holders to transfer their tokens to another address. It takes two parameters: the recipient address (`to`) and the amount of tokens to be transferred (`amount`). This function follows the standard ERC20 transfer behavior and returns a boolean value indicating the success of the transfer.

#### `redeem(uint256 amount)`

The `redeem` function allows token holders to burn their tokens in exchange for some specific functionality, such as in-game items or rewards. It takes one parameter: the amount of tokens to be burned (`amount`). This function burns the specified amount of tokens from the caller's address.

#### `burn(uint256 amount)`

The `burn` function allows anyone to burn tokens they own. It takes one parameter: the amount of tokens to be burned (`amount`). This function burns the specified amount of tokens from the caller's address.

#### `balanceOf(address account)`

##   
DegenToken Smart Contract

This is a Solidity smart contract for the DegenToken, an ERC20 token deployed on the Avalanche network. The contract extends the functionality of the OpenZeppelin `ERC20` and `Ownable` contracts, providing features such as minting, transferring, redeeming, burning, and checking token balances.

### Contract Details

-   Contract Name: DegenToken
-   Symbol: DGN
-   Decimals: 18
-   Initial Supply: 0

### Functions

#### `constructor()`

The constructor function initializes the token with the name "Degen" and the symbol "DGN". It is executed only once during contract deployment.

#### `mint(address to, uint256 amount)`

The `mint` function allows the owner of the contract to create and distribute new tokens. It takes two parameters: the recipient address (`to`) and the amount of tokens to be minted (`amount`). Only the contract owner is authorized to call this function.

#### `transfer(address to, uint256 amount)`

The `transfer` function allows token holders to transfer their tokens to another address. It takes two parameters: the recipient address (`to`) and the amount of tokens to be transferred (`amount`). This function follows the standard ERC20 transfer behavior and returns a boolean value indicating the success of the transfer.

#### `redeem(uint256 amount)`

The `redeem` function allows token holders to burn their tokens in exchange for some specific functionality, such as in-game items or rewards. It takes one parameter: the amount of tokens to be burned (`amount`). This function burns the specified amount of tokens from the caller's address.

#### `burn(uint256 amount)`

The `burn` function allows anyone to burn tokens they own. It takes one parameter: the amount of tokens to be burned (`amount`). This function burns the specified amount of tokens from the caller's address.

#### `balanceOf(address account)`

The `balanceOf` function allows users to check the token balance of a specific address. It takes one parameter: the address for which the balance should be queried. It returns the balance of the specified address.

### License

This smart contract is released under the MIT License, a permissive open-source license. Please see the SPDX-License-Identifier comment at the beginning of the contract for the full license text.
The `balanceOf` function allows users to check the token balance of a specific address. It takes one parameter: the address for which the balance should be queried. It returns the balance of the specified address.

### License

This smart contract is released under the MIT License, a permissive open-source license. Please see the SPDX-License-Identifier comment at the beginning of the contract for the full license text.gram
* Any modifications needed to be made to files/folders

