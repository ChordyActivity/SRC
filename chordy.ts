import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Chordy } from "../target/types/chordy";

describe("chordy", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Chordy as Program<Chordy>;

  it("Initialize Track", async () => {
    const track = anchor.web3.Keypair.generate();

    await program.methods
      .initializeTrack("AI Symphony", 10)
      .accounts({
        track: track.publicKey,
        creator: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([track])
      .rpc();

    console.log("Track Created:", track.publicKey.toString());
  });
});
