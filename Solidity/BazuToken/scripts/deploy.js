const hre = require("hardhat");

async function main() {
  const bazuToken = await hre.ethers.deployContract("BazuToken");

  await bazuToken.waitForDeployment();

  console.log(`BazuToken deployed to ${bazuToken.address}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
