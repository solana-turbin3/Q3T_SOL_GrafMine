import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { expect } from "chai";

describe("vault_anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;
  
  const vaultState = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("state"),
        provider.publicKey.toBytes()],
      program.programId)[0];
  
  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"),
        vaultState.toBytes()],
      program.programId)[0];
  
  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().accountsPartial(
        {
          user: provider.wallet.publicKey,
          state: vaultState,
          vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        }
    ).rpc();
    console.log("Your transaction signature", tx);
  });
  
  it("dep 2 sol!", async () => {
    const depositAmount = new anchor.BN(2000000);
    const initialBalance = await provider.connection.getBalance(vault);
    const tx = await program.methods.deposit(depositAmount).accountsPartial({
        user: provider.wallet.publicKey,
        vault,
        state: vaultState,
        systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
      console.log("Подпись транзакции депозита", tx);
      
      const finalBalance = await provider.connection.getBalance(vault);
      
      expect(finalBalance).to.equal(initialBalance + depositAmount.toNumber());
  });
  
  it("withdraw 1 sol!", async () => {
    const withdrawAmount = new anchor.BN(1000000);
      const withdrawTx = await program.methods.withdraw(withdrawAmount).accountsPartial({
          user: provider.wallet.publicKey,
          vault,
          state: vaultState,
          systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      console.log("Подпись транзакции снятия", withdrawTx);
  });
  
  it("close!", async () => {
      const closeTx = await program.methods.close().accountsPartial({
          user: provider.wallet.publicKey,
          vault,
          vaultState,
          systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      console.log("Подпись транзакции закрытия", closeTx);
      
      const vaultBalanceAfterClose = await provider.connection.getBalance(vault);
      expect(vaultBalanceAfterClose).to.equal(0);
      
      // Проверяем, что state аккаунт закрыт
      try {
          await program.account.vaultState.fetch(vaultState);
          throw new Error("State аккаунт все еще существует");
      } catch (error) {
          expect(error.message).to.include("Account does not exist");
      }
  });
});
