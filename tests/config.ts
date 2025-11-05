import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dex } from "../target/types/dex";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { expect } from "chai";

describe("config", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const swapFee = 30;
  const protocolFee = 20;

  const program = anchor.workspace.dex as Program<Dex>;
  const AnchorProvider = anchor.AnchorProvider.env();
  it("Is initialized!", async () => {
    const authorityKeypair = Keypair.generate();
    const authorityWallet = new anchor.Wallet(authorityKeypair);

    const protocolKeypair = Keypair.generate();
    const protocolWallet = new anchor.Wallet(protocolKeypair);

    const [configPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )

    const airdroptx = await AnchorProvider.connection.requestAirdrop(authorityWallet.publicKey,10 * LAMPORTS_PER_SOL)
    await AnchorProvider.connection.confirmTransaction(airdroptx,"confirmed");

    // Add your test here.
    const tx = await program.methods.initializeConfig(swapFee,protocolFee).
    accounts(
      {
        payer: authorityWallet.publicKey,
        admin: authorityWallet.publicKey,
        protocolFeeAccount: protocolWallet.publicKey,
        config: configPDA
      }
    ).
    signers(
      [authorityWallet.payer]
    ).
    rpc();

    await AnchorProvider.connection.confirmTransaction(tx,"confirmed");
    const configInfo = await program.account.configState.fetch(configPDA);
    console.log("Your transaction signature", tx);

    //验证数据是否正确
    expect(configInfo.admin.toString()).to.equal(authorityWallet.publicKey.toString());
    expect(configInfo.protocolFeeAccount.toString()).to.equal(protocolWallet.publicKey.toString());
    expect(configInfo.isPaused).to.equal(false);
    expect(configInfo.protocolFeeRate).to.equal(protocolFee);
    expect(configInfo.swapFeeRate).to.equal(swapFee);
  });
});
