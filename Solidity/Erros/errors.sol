// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint256 public value;

    function setValue(uint256 _newValue) public {
        // Using require() statement
        require(_newValue > value, "New value must be greater than the current value");
        
        value = _newValue;
    }
    
    function increaseValue(uint256 _amount) public {
        uint256 oldValue = value;
        
        // Using assert() statement
        assert(value + _amount >= value);
        
        value += _amount;
        
        // Using revert() statement
        if (value < oldValue) {
            revert("Value cannot be decreased");
        }
    }
}
