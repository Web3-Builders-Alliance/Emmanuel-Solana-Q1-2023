import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import {
  getAssociatedTokenAddress,
  createMint,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
} from "@solana/spl-token"
import { Deposit } from "../target/types/deposit"
import {Keypair, LAMPORTS_PER_SOL, PublicKey, Signer, SystemProgram} from "@solana/web3.js";

describe("deposit-spl", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Deposit as Program<Deposit>
  const connection = anchor.getProvider().connection
  const userWallet = anchor.workspace.Deposit.provider.wallet
  const myKeypair = Keypair.generate();

  console.log("Requesting SOL for userWallet...");
  // airdrop 1000000000 Lamports (10 SOL)
  const airdropUserWallet = await connection.requestAirdrop(myKeypair.publicKey, LAMPORTS_PER_SOL * 10);
  await connection.confirmTransaction(myKeypair.publicKey, "processed");
  console.log('solana public address: ' + myKeypair.publicKey.toBase58());

  it("Test Instruction", async () => {

    //create mint
    const mint = await createMint(connection, myKeypair, myKeypair.publicKey, myKeypair.publicKey, 6)

    console.log('mint public address: ' + mint.publicKey.toBase58());

    //get the token accont of this solana address, if it does not exist, create it
     const tokenAccountAddress = await mint.getOrCreateAssociatedAccountInfo(
      myKeypair.publicKey
    )

    console.log('token public address: ' + tokenAccountAddress.address.toBase58());

    console.log("Sending 200X to TokenAccount ...");
    // minting 200 new tokens to the token address we just created
    await mint.mintTo(tokenAccountAddress, myKeypair.publicKey, [], LAMPORTS_PER_SOL * 200);
    
  });

  it("Deposits spl tokens", async () => {
    const deposit_amount = new anchor.BN(25 * anchor.web3.LAMPORTS_PER_SOL);
    const deposit_native_tx = await program.methods.depositSpl(deposit_amount)
      .accounts({
        depositAccount: deposit_account.publicKey,
        pdaAuth: pda_auth,
        solVault: sol_vault,
        depositAuth: deposit_auth.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([deposit_auth]).rpc();

    let sol_vault_lamps = await provider.connection.getBalance(sol_vault);
    console.log(sol_vault_lamps);

    let result = await program.account.depositBase.fetch(deposit_account.publicKey);
    console.log(result);

  });
})