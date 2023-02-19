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
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";

describe("deposit-spl", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Deposit as Program<Deposit>
  const connection = anchor.getProvider().connection
  const userWallet = anchor.workspace.Deposit.provider.wallet

  console.log("Requesting SOL for userWallet...");
  // airdrop 1000000000 Lamports (10 SOL)
  const airdropUserWallet = await connection.requestAirdrop(userWallet.publicKey, LAMPORTS_PER_SOL * 10);
  await connection.confirmTransaction(airdropUserWallet, "processed");
  console.log(airdropUserWallet)

  it("Test Instruction", async () => {
    const mint = await createMint(
      connection,
      userWallet.payer,
      userWallet.publicKey,
      userWallet.publicKey,
      6
    )

    const tokenAccountAddress = await getAssociatedTokenAddress(
      mint,
      userWallet.publicKey
    )

    console.log("Sending 200X to TokenAccount ...");
    await mint.mintTo(tokenAccountAddress, userWallet.publicKey, [], LAMPORTS_PER_SOL * 200);

    console.log(userWallet.publicKey)
    //
    const metaplex = new Metaplex(connection);
    metaplex.use(keypairIdentity(userWallet));

    const mintNFTResponse = await metaplex.nfts().create({
      uri: "https://ffaaqinzhkt4ukhbohixfliubnvpjgyedi3f2iccrq4efh3s.arweave.net/KUAIIbk6p8oo4XHRcq0U__C2r0mwQaNl0gQow4Qp9yk",
      name: "Fancy token",
      maxSupply: 1,
    });

    /*
    const metaplexNFTs = Metaplex.make(connection)
    .use(keypairIdentity(userWallet))
    .use(bundlrStorage())
    .nfts()

    createOutput = await metaplexNFTs.create(
      {
        uri: metadataURL,
        name: "Some token name", <- I'd like to be able to add 'name' but also other metadata with @solana/spl-token
        sellerFeeBasisPoints: 0, 
      }
    );
    */
    
  })
})