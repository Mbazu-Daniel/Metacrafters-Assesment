import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, transfer } from '@solana/spl-token';
import bs58 from 'bs58'
import fs from 'fs'
(async () => {
    // Step 1: Connect to cluster and generate two new Keypairs
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    const secretkey = new Uint8Array(
        ["USE YOUR WALLET SECRET Uint8Array KEY"]
    )
    const toWalletKey = new Uint8Array(
        [ "USE YOUR WALLET SECRET Uint8Array KEY"]
    )

    const key = Keypair.generate();
    console.log(key)
  
    
    // console.log(toWallet)
    const fromWallet = Keypair.fromSecretKey(secretkey); 
    const toWallet = Keypair.fromSecretKey(toWalletKey) 

    console.log("from",fromWallet.publicKey.toString())
    console.log("to",toWallet.publicKey.toString())

    const mint = await createMint(connection, fromWallet, fromWallet.publicKey, null, 9);
    // It returns the public key of the token account
    // account to hold a balance of the new token
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        fromWallet, // payer
        mint, // mint account
        fromWallet.publicKey //owner
    )
    console.log(`From token address/(account address) is ${fromTokenAccount.address}`)
    // Set the token name
    //  await mint.setTokenName(tokenName);

    //Step 4: Mint a new token to the from account
    let signature = await mintTo(
        connection,
        fromWallet, // Payer
        mint, // Mint account
        fromTokenAccount.address, // Token account we are minting to
        fromWallet.publicKey, //Account to give authority
        5000000000, // 5 Tokens
        []
    );
    console.log("================================")
    console.log('mint tx:', signature);
    console.log("================================")
    console.log("Mint address is", mint.toString()) // Unique token identifier


    //Step 5: Get the token account of the to-wallet address and if it does not exist, create it
    const toTokenAccount = await getOrCreateAssociatedTokenAccount(connection, fromWallet, mint, toWallet.publicKey);
    console.log("================================")
    console.log(`To token address/(account address) is ${toTokenAccount.address}`)

    //Step 6: Transfer the new token to the to-wallet's token account that was just created
    // Transfer the new token to the "toTokenAccount" we just created
    signature = await transfer(
        connection,
        fromWallet,
        fromTokenAccount.address,
        toTokenAccount.address,
        fromWallet.publicKey,
        2000000000, // 2 tokens Equivalent to one token 1e9 decimal
        []
    );
    console.log("================================")
    console.log('transfer tx:', signature);


})();
