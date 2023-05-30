// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

//    REQUIREMENTS
// 1. Your contract will have public variables that store the details about your 
// coin (Token Name, Token Abbrv., Total Supply)
// 2. Your contract will have a mapping of addresses to balances (address => uint)
// 3. You will have a mint function that takes two parameters: an address and a value.
//    The function then increases the total supply by that number and increases the balance
//    of the “sender” address by that amount
// 4. Your contract will have a burn function, which works the opposite of 
// the mint function, as it will destroy tokens.
//    It will take an address and value just like the mint functions. 
// It will then deduct the value from the total supply
//    and from the balance of the “sender”.
// 5. Lastly, your burn function should have conditionals to make sure the balance
//  of "sender" is greater than or equal
//    to the amount that is supposed to be burned.

contract BazuToken {
    // public variables here
    string public constant TOKEN_NAME = "Bazu Token";
    string public constant TOKEN_SYMBOL = "BZT";
    uint256 public totaSupply = 0;

    // mapping variable here
    mapping(address => uint256) public balances;

    // mint function
    function mint(uint256 _value) public {
        totaSupply += _value;
        balances[msg.sender] += _value;
    }

    // burn function
    function burn(uint256 _value) public {
        if (balances[msg.sender] >= _value) {
            totaSupply -= _value;
            balances[msg.sender] -= _value;
        }
    }
}
