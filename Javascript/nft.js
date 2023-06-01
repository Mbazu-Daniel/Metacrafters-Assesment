// create a variable to hold your NFT's
let nfts = [];

// this function will take in some values as parameters, create an
// NFT object using the parameters passed to it for its metadata,
// and store it in the variable above.
function mintNFT(name, description, image) {
  const nft = {
    name: name,
    description: description,
    image: image,
  };
  nfts.push(nft);
}

// create a "loop" that will go through an "array" of NFT's
// and print their metadata with console.log()
function listNFTs() {
  for (let i = 0; i < nfts.length; i++) {
    console.log("Name: " + nfts[i].name);
    console.log("Description: " + nfts[i].description);
    console.log("Image: " + nfts[i].image);
    console.log("----------------------");
  }
}

// print the total number of NFTs we have minted to the console
function getTotalSupply() {
  return nfts.length;
}

// call your functions below this line

mintNFT("NFT 1", "Description for NFT 1", "image-url-1");
mintNFT("NFT 2", "Description for NFT 2", "image-url-2");
mintNFT("NFT 3", "Description for NFT 3", "image-url-3");
mintNFT("NFT 4", "Description for NFT 4", "image-url-4");

listNFTs();
console.log("Total Supply: " + getTotalSupply());
