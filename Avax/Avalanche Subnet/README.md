# DeFi Empire: DeFi Kingdom Clone on Avalanche

## Overview

Defi Empire is an exciting blockchain-based gaming experience where players can collect, build, and battle with their digital assets. By engaging in various game activities, players have the opportunity to earn rewards. We've developed this project as a DeFi Kingdom clone on the Avalanche network, merging decentralized finance and gaming to create an immersive universe.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Project Steps](#project-steps)
- [Smart Contracts](#smart-contracts)
- [Usage](#usage)
- [Author](#author)
- [License](#license)

## Prerequisites

Before you begin, ensure you have the following tools and resources:

- Unix computer (MacOS or Linux) or WSL installed on Windows
- Remix online IDE
- Metamask wallet extension
- Web browser

## Project Steps

1. **Set up your EVM subnet:** To create a custom EVM subnet on the Avalanche network, follow the instructions provided in the guide and refer to the [Avalanche documentation](https://docs.avax.network/learn/avalanche/subnets-overview). This will allow you to deploy your smart contracts with low fees and create a custom token.

2. **Define your native currency:** Create your own native currency to serve as the in-game currency for your DeFi Kingdom clone. This currency will be used for transactions, rewards, and various in-game activities.

3. **Connect to Metamask:** To connect your EVM Subnet to Metamask, follow the instructions in the provided guide. Make sure to select your custom EVM subnet as the network in Metamask.

4. **Deploy basic building blocks:** To deploy foundational smart contracts for your game, you can leverage Solidity and Remix. These contracts will define activities such as battling, exploring, and trading. You can find example contracts in the provided guide.

   - [ERC20 Token Contract](contracts/ERC20.sol)
   - [Vault Contract](contracts/Vault.sol)

5. **Test your application:** You can use Remix to interact with your deployed smart contracts for your DeFi Kingdom clone. With Remix, you can deploy tokens, liquidity pools, and other important components of your game. You can also test functionalities like battling, exploring, and trading within your game.

## Smart Contracts

1. **ERC20 Token Contract:** The contract is an implementation of a standard ERC20 token. It includes features such as transfers, allowances, minting, and burning. Players can utilize this token as an in-game currency.

   - [ERC20.sol](contracts/ERC20.sol)

2. **Vault Contract:** The Vault contract allows users to deposit and withdraw tokens while keeping track of the total supply and individual balances. It is used to manage liquidity within the game.

   - [Vault.sol](contracts/Vault.sol)

## Usage

To start your DeFi Kingdom adventure, follow these steps:

1. Set up your EVM subnet using the provided guide and Avalanche documentation.

2. Define your native currency within the game.

3. Connect your EVM Subnet to Metamask as outlined in the guide.

4. Deploy the foundational smart contracts using Remix.

5. Test your DeFi Kingdom clone by interacting with the deployed smart contracts through Remix.

## Author

[Michael](https://github.com/m-azra3l)

## License

This project is licensed under the [MIT License](LICENSE).
