import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DonationProgram } from "../target/types/donation_program";
import * as assert from "assert";

describe("donation-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));

  const program = anchor.workspace.DonationProgram as Program<DonationProgram>;
  const userWallet = anchor.workspace.Deposit.provider.wallet;
  /*
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
  */
  const [campaignAccount, _] = anchor.web3.PublicKey
      .findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("campaign"),
          userWallet.publicKey.toBuffer()
        ],
        program.programId
      );

  it('Should create campaign', async () => {
        await program.methods
            .create('test campaign', 'test description', new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL))
            .accounts({
                campaign: campaignAccount,
                user: userWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        const campaignAcc = await program.account.campaign.fetch(
            campaignAccount,
        );
        assert.equal(campaignAcc.name, 'test campaign');
        assert.equal(campaignAcc.description, 'test description');
        assert.ok(campaignAcc.targetAmount.eq(new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL)));
        assert.ok(campaignAcc.owner.equals(userWallet.publicKey));
        assert.ok(campaignAcc.amountDonated.eq(new anchor.BN(0)));
    });

     it('Should donate to campaign', async () => {
        await program.methods
            .donate(new anchor.BN(0.33 * anchor.web3.LAMPORTS_PER_SOL))
            .accounts({
                campaign: campaignAccount,
                user: userWallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        const campaignAcc = await pg.program.account.campaign.fetch(
            campaignAccount,
        );
        //console.log("campaignAcc.amountDonated data is:", campaignAcc.amountDonated.toString());
        assert.ok(campaignAcc.amountDonated.eq(new anchor.BN(0.33 * anchor.web3.LAMPORTS_PER_SOL)));
    });

    it('Should withdraw to owner wallet', async () => {
        await program.methods
            .withdraw(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL))
            .accounts({
                campaign: campaignAccount,
                user: userWallet.publicKey,
            })
            .rpc();
            
        const campaignAcc = await program.account.campaign.fetch(
            campaignAccount,
        );
        // Should be the same as before
        //console.log("campaignAcc.amountDonated 2 data is:", campaignAcc.amountDonated.toString());
        assert.ok(campaignAcc.amountDonated.eq(new anchor.BN(0.33 * anchor.web3.LAMPORTS_PER_SOL)));
    });
});
