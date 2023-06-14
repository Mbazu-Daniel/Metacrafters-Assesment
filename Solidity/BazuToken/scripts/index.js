const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);
  const bazuToken = await hre.ethers.deployContract("BazuToken");

  const deployerBalance = await bazuToken.balanceOf(deployer.address);
  console.log(
    "🚀 ~ file: index.js:9 ~ main ~ deployerBalance:",
    deployerBalance.toString()
  );

  const mintedToken = await bazuToken.connect(deployer).mint(1000);
  mintedToken.wait(10);
  console.log(
    "🚀 ~ file: index.js:9 ~ main ~ deployerBalance:",
    deployerBalance.toString()
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
