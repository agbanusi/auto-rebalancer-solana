import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AutoRebalancer } from "../target/types/auto_rebalancer";

describe("auto-rebalancer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AutoRebalancer as Program<AutoRebalancer>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
