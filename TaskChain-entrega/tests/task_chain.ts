import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("task_chain", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // The generated IDL type is supplied by Anchor after building the program.
  const program = anchor.workspace.TaskChain as Program<any>;
  const taskId = new anchor.BN(Date.now());

  const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("task"),
      provider.wallet.publicKey.toBuffer(),
      taskId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  it("creates, updates, completes and deletes a task", async () => {
    await program.methods
      .createTask(taskId, "First task", "TaskChain test")
      .accounts({
        task: taskPda,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    let task = await program.account.task.fetch(taskPda);
    assert.equal(task.title, "First task");
    assert.equal(task.completed, false);

    await program.methods
      .updateTask("Updated task", "Updated description")
      .accounts({
        task: taskPda,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    task = await program.account.task.fetch(taskPda);
    assert.equal(task.title, "Updated task");

    await program.methods
      .completeTask()
      .accounts({
        task: taskPda,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    task = await program.account.task.fetch(taskPda);
    assert.equal(task.completed, true);

    await program.methods
      .deleteTask()
      .accounts({
        task: taskPda,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    try {
      await program.account.task.fetch(taskPda);
      assert.fail("The task account should have been closed");
    } catch (error) {
      assert.include(String(error).toLowerCase(), "account");
    }
  });
});
