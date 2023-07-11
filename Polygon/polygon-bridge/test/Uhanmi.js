const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BazRiotNFT", function () {
  let owner, user1, user2;
  let contract;

  beforeEach(async function () {
    // Get signers
    [owner, user1, user2] = await ethers.getSigners();

    // Deploy contract
    const BazRiotNFT = await ethers.getContractFactory("BazRiotNFT");
    contract = await BazRiotNFT.deploy();
    await contract.deployed();
  });

  // Test that the prompt description is set correctly
  it("should set the correct prompt description", async function () {
    expect(await contract.promptDescription()).to.equal(
      "https://gateway.pinata.cloud/ipfs/QmZF5S33oXteEZi5RpsQipt2UdmZ6YmENx32p6kTLaQpXR"
    );
  });

  // Test that the owner can mint tokens
  it("should allow the owner to mint tokens", async function () {
    await contract.connect(owner).mint(5);

    expect(await contract.totalSupply()).to.equal(5);
    expect(await contract.balanceOf(owner.address)).to.equal(5);
  });

  // Test that non-owners cannot mint tokens
  it("should not allow non-owners to mint tokens", async function () {
    await expect(contract.connect(user1).mint(5)).to.be.revertedWith(
      "Only owner can perform this action!"
    );

    expect(await contract.totalSupply()).to.equal(0);
    expect(await contract.balanceOf(user1.address)).to.equal(0);
  });

  // Test that minting more than the maximum quantity is not allowed
  it("should not allow minting more than the maximum quantity", async function () {
    await contract.connect(owner).mint(5);

    await expect(contract.connect(owner).mint(1)).to.be.revertedWith(
      "You can not mint more than 5"
    );

    expect(await contract.totalSupply()).to.equal(5);
    expect(await contract.balanceOf(owner.address)).to.equal(5);
  });
});
