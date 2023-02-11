import * as anchor from "@project-serum/anchor";
import { BN, Program, Wallet } from "@project-serum/anchor";
import { Deposit } from "../target/types/deposit";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, Signer, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("deposit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Deposit as Program<Deposit>;
  const depositOwner = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  const [depositAccount, _] = PublicKey
      .findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("deposit-state"),
          depositOwner.publicKey.toBuffer()
        ],
        program.programId
      );

      it('Should initialize', async () => {
        await program.methods
            .initialize('test deposit', new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL))
            .accounts({
                depositState: depositAccount,
                user: user.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();
        
        const depositAcc = await program.account.depositState.fetch(
          depositAccount,
        );
        assert.equal(depositAcc.name, 'test deposit');
        assert.ok(depositAcc.targetAmount.eq(new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL)));
        assert.ok(depositAcc.owner.equals(depositOwner.publicKey));
        assert.ok(depositAcc.amountDeposited.eq(new anchor.BN(0)));
        
    });

    it('Should deposit', async () => {
      await program.methods
          .deposit(new anchor.BN(0.33 * anchor.web3.LAMPORTS_PER_SOL))
          .accounts({
            depositState: depositAccount,
              user: user.publicKey,
              systemProgram: SystemProgram.programId,
          })
          .rpc();

      const depositAcc = await program.account.depositState.fetch(
        depositAccount,
      );
      assert.ok(depositAcc.amountDeposited.eq(new anchor.BN(0.33 * anchor.web3.LAMPORTS_PER_SOL)));
  });


  it('Should withdraw to owner wallet', async () => {
    await program.methods
        .withdraw(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL))
        .accounts({
          depositState: depositAccount,
            user: depositOwner.publicKey,
        })
        .rpc();
        
    const depositAcc = await program.account.depositState.fetch(
      depositAccount,
    );
    // Should be the same as before i.e 0.33 - 0.1 = 0.23 
    assert.ok(depositAcc.amountDeposited.eq(new anchor.BN(0.23 * anchor.web3.LAMPORTS_PER_SOL)));
});

});
