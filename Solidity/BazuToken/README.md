

# Bazu Token
Bazu Token is an ERC-20 compliant token implemented on the Ethereum blockchain. It has the following features:

Token Name: Bazu Token
Token Symbol: BZT
Decimals: 18
Total Supply: 1,000 BZT

# Contract Details
The contract is implemented in Solidity and has the following functions:

`constuctor()`
The constructor function is executeonly once during the contract deployment. It initializes the contract owner, token name, token symbol, decimals, and total supply.

`transfer(address _to, uint256 _value)`
This function allows users to transfer tokens from their own address to the specified _to address. It verifies that the sender has a sufficient balance and updates the balances accordingly. It emits a Transfer event to notify listeners about the token transfer.

`burn(uint256 _value)`
The burn function allows users to burn (destroy) a specific amount of their own tokens. It verifies that the sender has a sufficient balance and reduces both their balance and the total supply accordingly. It emits a Burn event to notify listeners about the token burn.

`mint(uint256 _value)`
The mint function can only be called by the contract owner. It allows the owner to mint (create) new tokens and add them to their own balance. The total supply is increased accordingly, and a Transfer event is emitted to notify listeners about the token minting.

# Events
The contract defines two events:

`Transfer(address indexed from, address indexed to, uint256 value)`
This event is emitted when tokens are transferred from one address to another. It provides information about the sender, recipient, and the amount of tokens transferred.

`Burn(address indexed from, uint256 value)`
This event is emitted when tokens are burned (destroyed). It provides information about the address from which the tokens are burned and the amount of tokens burned.

