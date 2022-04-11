const fs = require("mz/fs");
const solana = require("@solana/web3.js");

async function createKeypairFromFile(filePath) {
    const secretKeyString = await fs.readFile(filePath, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));

    return solana.Keypair.fromSecretKey(secretKey);
}

exports.createKeypairFromFile = createKeypairFromFile;
