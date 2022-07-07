import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EditionsProgram } from "../editions/typescript";

describe("editions", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Editions as Program<Editions>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
