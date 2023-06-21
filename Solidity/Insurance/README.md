

# Insurance Contract

This repository contains three Solidity contracts: `Wallet.sol`, `CollateralProtection.sol`, and `InsuranceFactory.sol`. These contracts provide functionality for creating and managing insurance policies and wallets.

## Contracts

### Wallet.sol

The `Wallet` contract implements a basic insurance wallet. It allows users to pay for insurance coverage and claim their insurance when the coverage period expires. The contract includes the following features:

-   **Owner**: The contract owner who deploys the contract and manages the insurance.
-   **Insured**: A flag indicating whether the user is insured or not.
-   **InsuredAmount**: The amount required to be paid for insurance coverage.
-   **Token**: A value representing the insurance coverage tokens.
-   **CoverageTime**: The timestamp indicating the end of the coverage period.
-   **WalletBalance**: A mapping of addresses to their wallet balances.
-   **WalletTokenBalance**: A mapping of addresses to their wallet token balances.

The `Wallet` contract provides the following functions:

-   `payInsurance()`: Allows users to pay for insurance coverage by sending the required amount of Ether.
-   `claimInsurance()`: Allows the owner to claim their insurance once the coverage period has expired.
-   `getWalletBalance()`: Retrieves the wallet balance for the caller's address.
-   `getWalletTokenBalance()`: Retrieves the wallet token balance for the caller's address.

### CollateralProtection.sol

The `CollateralProtection` contract implements a collateral-based loan protection system. It allows the contract owner to create loans and collect collateral. The contract includes the following features:

-   **Owner**: The contract owner who deploys the contract and manages the loans.
-   **CollateralThreshold**: The minimum collateral threshold required for a loan.
-   **LoanCollateral**: A mapping of addresses to their collateral amounts.
-   **Loans**: A mapping of addresses to their loan policies.

The `CollateralProtection` contract provides the following functions:

-   `createLoan(uint256 amount, uint256 collateralAmount)`: Allows the owner to create a loan with the specified amount and collateral.
-   `collectLoan()`: Allows the owner to collect the loan amount if the collateral meets the threshold.
-   `payLoan()`: Allows users to repay their loan by sending the required amount of Ether.

### InsuranceFactory.sol

The `InsuranceFactory` contract acts as a factory for creating insurance-related contracts. It allows users to create crypto wallet insurance and collateral protection insurance contracts. The contract includes the following features:

-   **UserInsurance**: A mapping of addresses to their insurance contract addresses.
-   **UserWallet**: A mapping of addresses to their crypto wallet contract addresses.

The `InsuranceFactory` contract provides the following functions:

-   `createCryptoWalletInsurance(uint256 insuredAmount)`: Allows users to create a crypto wallet insurance contract with the specified insured amount.
-   `createCollateralProtectionInsurance()`: Allows users to create a collateral protection insurance contract.
-   `getUserInsurance()`: Retrieves the insurance contract address for the caller's address.
-   `getUserCryptoWalletContract()`: Retrieves the crypto wallet contract address for the caller's address.



## How to run this project 

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.js// SPDX-License-Identifier: MIT
