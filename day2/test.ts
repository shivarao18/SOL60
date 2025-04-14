it("Performs sqrt", async () => {
    const [resultPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("result"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  
    await program.methods.sqrt(new anchor.BN(144))
      .accounts({
        result: resultPda,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  
    const resultAccount = await program.account.calculatorResult.fetch(resultPda);
    console.log("SQRT result:", resultAccount.value.toString()); // should print 12
  });
  