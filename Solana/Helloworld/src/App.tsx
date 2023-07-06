// import functionalities
import React, { useEffect, useState } from 'react';
import {
  PublicKey,
  Transaction,
  Connection,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// create types
type DisplayEncoding = "utf8" | "hex";

type PhantomEvent = "disconnect" | "connect" | "accountChanged";
type PhantomRequestMethod =
  | "connect"
  | "disconnect"
  | "signTransaction"
  | "signAllTransactions"
  | "signMessage";

interface ConnectOpts {
  onlyIfTrusted: boolean;
}

// create a provider interface (hint: think of this as an object) to store the Phantom Provider
interface PhantomProvider {
  publicKey: PublicKey | null;
  isConnected: boolean | null;
  signTransaction: (transaction: Transaction) => Promise<Transaction>;
  signAllTransactions: (transactions: Transaction[]) => Promise<Transaction[]>;
  signMessage: (
    message: Uint8Array | string,
    display?: DisplayEncoding
  ) => Promise<any>;
  connect: (opts?: Partial<ConnectOpts>) => Promise<{ publicKey: PublicKey }>;
  disconnect: () => Promise<void>;
  on: (event: PhantomEvent, handler: (args: any) => void) => void;
  request: (method: PhantomRequestMethod, params: any) => Promise<unknown>;
}

/**
 * @description gets Phantom provider, if it exists
 */
const getProvider = (): PhantomProvider | undefined => {
  if ("solana" in window) {
    const provider = window.solana as any;
    if (provider.isPhantom) return provider as PhantomProvider;
  }
};

export default function App() {
  // create state variable for the provider
  const [provider, setProvider] = useState<PhantomProvider | undefined>(undefined);

  // create state variable for the wallet key
  const [walletKey, setWalletKey] = useState<string | undefined>(undefined);

  // create state variable to track the status of airdrop
  const [airdrop, setAirdrop] = useState<boolean>(false);

  // create state variable to track the status of transfer
  const [isTransfer, setIsTransfer] = useState<boolean>(false);

  // Create a state variable to store the created wallet
  const [createdWallet, setCreatedWallet] = useState<string | undefined>(undefined);

  // Create a state variable to store the created wallet
  const [UserPrivateKey, setUserPrivateKey] = useState<Buffer | undefined>(undefined);

  // Store balance of wallets
  const [walletBalance, setWalletBalance] = useState<number | undefined>(undefined);
  const [userBalance, setUserBalance] = useState<number | undefined>(undefined);

  // this is the function that runs whenever the component updates (e.g. render, refresh)
  useEffect(() => {
    const provider = getProvider();

    // if the phantom provider exists, set this as the provider
    if (provider) setProvider(provider);
    else setProvider(undefined);
  }, []);

  /**
   * @description prompts user to connect wallet if it exists.
   * This function is called when the connect wallet button is clicked
   */
  const connectWallet = async () => {
    const { solana } = window;

    // checks if phantomwallet exists
    if (solana) {
      try {
        // connects wallet and returns response which includes the wallet public key
        const response = await solana.connect();
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        console.log('wallet account ', response.publicKey.toString());
        // update walletKey to be the public key
        setWalletKey(response.publicKey.toString());

        // get wallet balance
        const walletBalance = await connection.getBalance(
          new PublicKey(response.publicKey.toString())
        );
        setWalletBalance(walletBalance);
      } catch (err) {
        // { code: 4001, message: 'User rejected the request.' }
      }
    }
  };

  const disconnectWallet = async () => {
    const { solana } = window;
    if (solana) {
      try {
        const response = await solana.disconnect();
        setWalletKey(undefined);
      } catch (err) {

      }
    }
  }


  const createWallet = () => {
    const { solana } = window;
    if (solana) {
      try {
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        const newPair = new Keypair();
        const publicKey = new PublicKey(newPair.publicKey).toString();
        const privateKey = newPair.secretKey;

        setCreatedWallet(publicKey);
        setUserPrivateKey(privateKey);
      } catch (err) {

      }
    }
  }

  const tokenAirdrop = async () => {
    const { solana } = window;
    if (solana) {
      try {
        // Connect to the Devnet and make a wallet from privateKey
        const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        const myWallet = await Keypair.fromSecretKey(UserPrivateKey);

        // Request airdrop of 2 SOL to the wallet
        console.log("Airdropping some SOL to my wallet!");
        const fromAirDropSignature = await connection.requestAirdrop(
          new PublicKey(createdWallet),
          2 * LAMPORTS_PER_SOL
        );
        await connection.confirmTransaction(fromAirDropSignature);
        setAirdrop(true);
        getUserBalance();
      } catch (err) {

      }
    }
  }

  const getUserBalance = async () => {
    const { solana } = window;
    if (solana) {

      const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

      // get wallet balance
      const walletBalance = await connection.getBalance(
        new PublicKey(createdWallet)
      );
      setUserBalance(walletBalance)
    }
  }

  // transfer Sol
  const transferToken = async () => {
    console.log(`Im here`);
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const from = Keypair.fromSecretKey(UserPrivateKey);
    const to = new PublicKey(walletKey);

    console.log(`Im about to compile transaction`);
    console.log(`new wallet balance : ${to}`)

    // Send money from "NEW" wallet and into "to" Phantom wallet
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: (userBalance - (1.9 * LAMPORTS_PER_SOL)),
      })
    );
    console.log(`I've compiled`);

    // Sign transaction
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);

    const senderBalanceAfter = await connection.getBalance(from.publicKey);
    setUserBalance(senderBalanceAfter);
    const receiverBalanceAfter = await connection.getBalance(to.publicKey);
    setWalletBalance(receiverBalanceAfter);
    setIsTransfer(true);

  }

  // HTML code for the app
  return (
    <div className="App">
      <header className="App-header">
        <div className="flex flex-row gap-4 w-full">
          <div className="w-1/2">
            {provider && walletKey && (
              <div>
                <button
                  className="bg-blue-500 text-white px-4 py-2 font-bold rounded"
                  onClick={disconnectWallet}
                >
                  Disconnect
                </button>
              </div>
            )}
            <h2>{!walletKey ? `Connect to Phantom Wallet` : `Connected to Phantom Wallet`}</h2>
            {provider && !walletKey && (
              <button
                className="bg-blue-500 text-white px-4 py-2 font-bold rounded"
                onClick={connectWallet}
              >
                Connect Wallet
              </button>
            )}
            {provider && walletKey && (
              <div>
                <p>Connected account:</p>
                <p>{walletKey}</p>
                <p>BALANCE:</p>
                <p>{walletBalance ? `${parseInt(walletBalance) / LAMPORTS_PER_SOL} SOL` : `00`}</p>
              </div>
            )}
            {!provider && (
              <p>
                No provider found. Install{" "}
                <a href="https://phantom.app/">Phantom Browser extension</a>
              </p>
            )}
          </div>
          <div>
            {!createdWallet && (
              <div>
                <h2>Create New Wallet</h2>
                <button
                  className="bg-blue-500 text-white px-4 py-2 font-bold rounded"
                  onClick={createWallet}
                >
                  Create Wallet
                </button>
              </div>
            )}
            {createdWallet && !airdrop && (
              <div>
                <h2>Airdrop Tokens To Wallet</h2>
                <p>{createdWallet}</p>
                <button
                  className="bg-blue-500 text-white px-4 py-2 font-bold rounded"
                  onClick={tokenAirdrop}
                >
                  Airdrop Tokens
                </button>
              </div>
            )}
            {airdrop && (
              <div>
                <h2>Tokens Airdropped!</h2>
                <p>Airdrop successful!</p>
                <p>BALANCE:</p>
                <p>{userBalance ? `${parseInt(userBalance) / LAMPORTS_PER_SOL} SOL` : `00`}</p>
              </div>
            )}
            {createdWallet && airdrop && (
              <div>
                <h2>Transfer Tokens</h2>
                <button
                  className="bg-blue-500 text-white px-4 py-2 font-bold rounded"
                  onClick={transferToken}
                >
                  Transfer Tokens
                </button>
              </div>
            )}
            {isTransfer && (
              <div>
                <h2>Tokens Transferred!</h2>
                <p>Transfer successful!</p>
                <p>Sender Balance:</p>
                <p>{userBalance ? `${parseInt(userBalance) / LAMPORTS_PER_SOL} SOL` : `00`}</p>
                <p>Receiver Balance:</p>
                <p>{walletBalance ? `${parseInt(walletBalance) / LAMPORTS_PER_SOL} SOL` : `00`}</p>
              </div>
            )}
          </div>
        </div>
      </header>
    </div>
  );
}
