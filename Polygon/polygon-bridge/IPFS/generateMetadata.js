const fs = require('fs');
const path = require('path');

const metadata = path.join(__dirname, 'metadata');
if (!fs.existsSync(metadata)) {
    fs.mkdirSync(metadata);
}

console.log(__dirname);

for (let i = 1; i <= 5; i++) {
    const json = {
        name: `BAZ RIOT NFT #${i}`,
        description: `Image of the Aba women Riot in Nigeria #${i}`,
        image: `https://gateway.pinata.cloud/ipfs/QmbUdNdT7B845V9vJM74mHFV9EfGi6ZL3XzRqVvC1ACu4W/pinata${i}.jpg`
    };

    fs.writeFileSync(path.join(metadata, String(i)), JSON.stringify(json));
}
console.log("metadata successfully generated")