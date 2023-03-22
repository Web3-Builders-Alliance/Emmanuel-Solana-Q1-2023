import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BinaryOptions } from "../target/types/binary_options";

describe("binary-options", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BinaryOptions as Program<BinaryOptions>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
