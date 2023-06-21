// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./Wallet.sol";
import "./CollateralProtection.sol";

contract InsuranceFactory {
    mapping(address => address) private userInsurance;
    mapping(address => address) private userWallet;

    event WalletCreated(address indexed user, address insuranceContract);
    event InsuranceCollateralCreated(address indexed user, address insuranceContract);

    function createCryptoWalletInsurance(uint256 insuredAmount) external {
        require(userWallet[msg.sender] == address(0), "User have an insurance account");

        Wallet walletContract = new Wallet(insuredAmount);
        userWallet[msg.sender] = address(walletContract);

        emit WalletCreated(msg.sender, address(walletContract));
    }

    function createCollateralProtectionInsurance() external {
        require(userInsurance[msg.sender] == address(0), "User have an insurance account");

        CollateralProtection collateralContract = new CollateralProtection();
        userInsurance[msg.sender] = address(collateralContract);

        emit InsuranceCollateralCreated(msg.sender, address(collateralContract));
    }

    function getUserInsurance() external view returns (address) {
        return userInsurance[msg.sender];
    }

    function getUserCryptoWalletContract() external view returns (address) {
        return userWallet[msg.sender];
    }
}
