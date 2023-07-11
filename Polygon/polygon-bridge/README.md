# Polygon-Advanced-Module-1

This is the first project in Polygon-Advance, in this project I was tasked deploy an NFT collection on the Ethereum blockchain, Map the collection to Polygon, and Transfer assets over via the Polygon Bridge.

## Getting Started

### Executing program

Download the codes by downloading the entire repository which will give you access to other contencts of the repository. Navigate to the ERC721A project directory,  run:

```shell

 yarn install

```

After installing the dependences, run the test file by using the following command:

```shell
yarn hardhat test
```

### Deploying the ERC721A Contract

Before deploying, make sure to rename ".env.example" to ".env" and provide your wallet private key where required i.e "PRIVATE_KEY= 'your wallet private key'". Run the following command to deploy the ERC721A contract to the Goerli Ethereum Testnet:

``` shell
yarn hardhat run scripts/deploy.js --network goerli 
```

The script will deploy the contract and provide the address of the contract in the console and also in the "contractAddress.js" file in the metadata folder.

### Batch Mint NFTs

Run the following command to batch mint NFTs using the deployed ERC721 contract:

``` shell
yarn hardhat run scripts/batchMint.js --network goerli
```

The script will mint the specified number of NFTs and assign them to your address.

### Approve and Deposit NFTs to Polygon Mumbai

Run the following commands to approve and deposit the minted NFTs from Ethereum to the Polygon Mumbai network using the FxPortal Bridge:

```shell
yarn hardhat run scripts/approveDeposit.js --network goerli
```

### Viewing the Prompt

To view the prompt for generating the images of the NFTs, run:

```shell
yarn hardhat run scripts/viewPrompt.js
```

## Author

[Michael](https://github.com/m-azra3l)

## License

This project is licensed under the [MIT License](LICENSE).
You can make a copy of the project to use for your own purposes.
