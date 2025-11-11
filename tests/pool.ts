import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dex } from "../target/types/dex";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { expect } from "chai";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { BN } from "bn.js";

describe("pool", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const swapFee = 30;
  const protocolFee = 20;

  const program = anchor.workspace.dex as Program<Dex>;
  const AnchorProvider = anchor.AnchorProvider.env();
  it("initialize pool!", async () => {
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
        // @ts-ignore
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

    //初始化流动性池

    //创建两个代币
    const tokenMintX = await createMint(
      AnchorProvider.connection,
      authorityWallet.payer,
      authorityWallet.publicKey,
      authorityWallet.publicKey,
      9,
    )
    const tokenMintY = await createMint(
      AnchorProvider.connection,
      authorityWallet.payer,
      authorityWallet.publicKey,
      authorityWallet.publicKey,
      9,
    )
    console.log("Token Mint X: ",tokenMintX);
    console.log("Token Mint Y: ",tokenMintY);
    //关联账户的创建
    const userTokenX = await getOrCreateAssociatedTokenAccount(
      AnchorProvider.connection,
      authorityWallet.payer,
      tokenMintX,
      authorityWallet.publicKey,
    )
    const userTokenY = await getOrCreateAssociatedTokenAccount(
      AnchorProvider.connection,
      authorityWallet.payer,
      tokenMintY,
      authorityWallet.publicKey,
    )
    console.log("Token ATA X: ",userTokenX);
    console.log("Token ATA Y: ",userTokenY);

    //铸造代币
    await mintTo(
      AnchorProvider.connection,
      authorityWallet.payer,
      tokenMintX,
      userTokenX.address,
      authorityWallet.payer,
      1000 * 10 ** 9, // 铸造 1000 个代币（考虑 9 位小数）
    )
    await mintTo(
      AnchorProvider.connection,
      authorityWallet.payer,
      tokenMintY,
      userTokenY.address,
      authorityWallet.payer,
      2000 * 10 ** 9, // 铸造 1000 个代币（考虑 9 位小数）
    )
    //查找池子的PDA地址
    const [poolPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"),tokenMintX.toBuffer(),tokenMintY.toBuffer()],
      program.programId
    )
    //查找池子中tokenX的PDA
    const [vaultXPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),poolPDA.toBuffer(),tokenMintX.toBuffer()],
      program.programId
    )
    //查找池子中tokenY的PDA
    const [vaultYPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),poolPDA.toBuffer(),tokenMintY.toBuffer()],
      program.programId
    )
    const initializePooltx = await program.methods.initializePool(
      new BN(400 * 10 ** 9),
      new BN(800 * 10 ** 9),
    ).accounts(
      {
        payer: authorityWallet.publicKey,
        //@ts-ignore
        config: configPDA,
        pool: poolPDA,
        tokenXMint: tokenMintX,  // camelCase，不是 token_x_mint
        tokenYMint: tokenMintY,  // camelCase，不是 token_y_mint
        // tokenXVault 和 tokenYVault 是 PDA，依赖于 pool
        // Anchor 应该能够自动推导，但由于它们依赖于另一个 PDA (pool)，
        // 我们可以手动提供，或者让 Anchor 自动推导
        tokenXVault: vaultXPDA,  // camelCase，不是 token_x_vault
        tokenYVault: vaultYPDA,  // camelCase，不是 token_y_vault
        userTokenX: userTokenX.address,  // camelCase，不是 user_token_x
        userTokenY: userTokenY.address,  // camelCase，不是 user_token_y
      })
      .signers([authorityWallet.payer])
      .rpc();

    await AnchorProvider.connection.confirmTransaction(initializePooltx,"confirmed");

    //获取池子的信息
    const poolinfo = await program.account.poolState.fetch(poolPDA);
    console.log("poolinfo: ",poolinfo);

    expect(poolinfo.tokenXMint.toString()).to.equal(tokenMintX.toString());
    expect(poolinfo.tokenYMint.toString()).to.equal(tokenMintY.toString());
    expect(poolinfo.tokenXVault.toString()).to.equal(vaultXPDA.toString());
    expect(poolinfo.tokenYVault.toString()).to.equal(vaultYPDA.toString());
    //验证池子token余额
    const vaultXBalance = await AnchorProvider.connection.getTokenAccountBalance(vaultXPDA);
    expect(vaultXBalance.value.amount.toString()).to.equal(String(400 * 10 ** 9));
    const vaultYBalance = await AnchorProvider.connection.getTokenAccountBalance(vaultYPDA);
    expect(vaultYBalance.value.amount.toString()).to.equal(String(800 * 10 ** 9));
    //验证用户余额
    const userXBalance = await AnchorProvider.connection.getTokenAccountBalance(userTokenX.address);
    expect(userXBalance.value.amount.toString()).to.equal(String(600 * 10 ** 9))
    const userYBalance = await AnchorProvider.connection.getTokenAccountBalance(userTokenY.address);
    expect(userYBalance.value.amount.toString()).to.equal(String(1200 * 10 ** 9))
  });
});
