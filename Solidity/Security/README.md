
# Findings:

1. The constructor function should be explicitly defined using the constructor keyword instead of the contract name, as it is deprecated in Solidity 0.8.x.
Suggestions:

Update the constructor function to use the constructor keyword:

`
constructor() {
    owner = msg.sender;
}
`

2. Add the `memory` keyword when declaring the str variable in the store function to indicate that it is a temporary variable residing in memory.

`
function store(uint256 _amount) public {
    Storage memory str = storages[msg.sender];
    str.user = msg.sender;
    str.amount = _amount;
}
`

# Fixes:

1. Updated the constructor to use the constructor keyword.
2. Added the memory keyword when declaring the str variable in the store function.


# Audit Report

1. The contract is vulnerable to a reentrant attack. This is because the store() function does not use the msg.sender modifier. This means that an attacker could call the store() function recursively, causing the contract to run out of gas.

2.  The contract does not have a way to recover funds that have been sent to it. This is because the store() function does not have a revert() statement. This means that if an attacker sends funds to the contract, there is no way to get them back.


To fix the reentrant attack vulnerability, I would add the msg.sender modifier to the store() function. This would prevent an attacker from calling the store() function recursively.

To fix the lack of a way to recover funds, I would add a revert() statement to the store() function. This would allow users to get their funds back if they send them to the contract by mistake.






# Sample Hardhat Project

This project demonstrates a basic Hardhat use case. It comes with a sample contract, a test for that contract, and a script that deploys that contract.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.js
```
