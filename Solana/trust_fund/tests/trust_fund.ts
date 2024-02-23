import { Provider } from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Keypair } from '@solana/web3.js';
import * as assert from 'assert';
import { createEscrow, withdraw } from "../target/types/trust_fund";

describe('Trust Fund Program', () => {
  // Instantiate the provider
  const provider = Provider.local();
  anchor.setProvider(provider);

  it('Should create and withdraw from escrow', async () => {
    // Generate program's keypairs
    const programKeypair = Keypair.generate();
    const escrowAccount = Keypair.generate();

    // Instantiate the program
    const program = new Program(idl, programKeypair);

    // Create an escrow
    const creator = Keypair.generate();
    const beneficiary = Keypair.generate();
    const amount = 1000; // Example amount
    const timelock = Math.floor(Date.now() / 1000) + 3600; // Timelock in 1 hour

    await program.rpc.createEscrow(timelock, {
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        creator: creator.publicKey,
        beneficiary: beneficiary.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
      signers: [escrowAccount, creator],
      instructions: [
        await createEscrow({
          amount,
        }),
      ],
    });

    // Withdraw from escrow
    const owner = creator; // In this example, owner is the creator
    const beforeBalance = await provider.connection.getBalance(beneficiary.publicKey);
    await program.rpc.withdraw({
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        owner: owner.publicKey,
        beneficiary: beneficiary.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      },
      signers: [owner],
      instructions: [withdraw()],
    });
    const afterBalance = await provider.connection.getBalance(beneficiary.publicKey);

    // Check if beneficiary's balance increased by amount
    assert.equal(afterBalance - beforeBalance, amount);
  });
});
