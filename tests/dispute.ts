import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Dispute } from "../target/types/dispute";

describe("dispute", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Dispute as Program<Dispute>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
