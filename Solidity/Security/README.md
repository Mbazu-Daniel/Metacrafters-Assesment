# Audit Report on the Security of the StorageVictim Contract using Slither:

* Uninitialized Pointer:
Within the store function of the contract, there is an uninitialized pointer Storage str that points to the storage address 0 (owner). This can result in unexpected behavior and potential security risks. Malicious actors could exploit this uninitialized pointer to manipulate data or gain unauthorized access.

Recommended Solution:
To resolve this issue, it is advised to initialize the `Storage` struct directly in the `store` function and assign the values to `storages[msg.sender]`. Here is the updated code:

`solidity
function store(uint256 _amount) public {
    storages[msg.sender] = Storage(msg.sender, _amount);
}
`
* Deprecated Constructor:
The contract currently employs a constructor with the same name as the contract, namely `function StorageVictim()`. However, this naming convention has been deprecated in Solidity 0.8.18. Utilizing the new constructor keyword ensures improved code clarity and compatibility with the latest version of Solidity.

Recommended Solution:
Rename the constructor from function `StorageVictim()` to constructor:

`constructor() {
    owner = msg.sender;
}`

* State Variables Visibility:
The owner variable is defined as an address but lacks a visibility specifier. In Solidity 0.8.18, it is essential for state variables to explicitly declare their visibility to prevent unintended access or modification.
Recommended Solution:
If the `owner` variable should only be accessed within the contract, declare it as `immutable`:

`address immutable owner;`

The TestNet being used is Polygan Mumbai.


By implementing these suggested fixes, the contract's security will be enhanced, and it will become compatible with Solidity 0.8.18. These changes address the identified issues and help reduce potential vulnerabilities.
