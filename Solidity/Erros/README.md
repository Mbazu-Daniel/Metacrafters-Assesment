
# Counter Contract

This contract implements a simple counter functionality in Solidity.

## Prerequisites

-   Solidity version 0.8.0 or higher

## Usage

The `Counter` contract provides the following functions:

### `setValue(uint256 _newValue)`

This function sets the value of the counter to the specified `_newValue` parameter. It uses the `require()` statement to ensure that the new value is greater than the current value. If the condition is not met, an error message is returned.

### `increaseValue(uint256 _amount)`

This function increases the value of the counter by the specified `_amount` parameter. It uses the `assert()` statement to check for overflow when adding the amount to the current value. If an overflow occurs, an error is thrown.

Additionally, this function uses the `revert()` statement to prevent the value from being decreased. If the new value is less than the old value, an error message is returned.
