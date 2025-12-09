import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MessageProgram } from "../target/types/message"

describe("message-program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  //configuring program and Account
  const program = anchor.workspace.messageProgram as Program<MessageProgram>;
  const messageAccount = anchor.web3.Keypair.generate();
  const authority = (program.provider as anchor.AnchorProvider).wallet.publicKey;

  //test case for initializing
  it("initialized!", async () => {
    const tx = await program.methods.initialize().accounts({
      messageAccount: messageAccount.publicKey,
      authority: authority,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([messageAccount]).rpc();
    console.log(" transaction signature", tx);
  });
  //test case for updating message
  it("update message", async () => {
    await program.methods.updateMessage("hello world")
      .accounts({
        messageAccount: messageAccount.publicKey,
        authority
      })
      .rpc();
  });


});
