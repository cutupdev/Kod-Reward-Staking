import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { StakingProgram } from '../target/types/staking_program';
import { SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";

import { Token, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import bs58 from 'bs58';

const PublicKey = anchor.web3.PublicKey;
const BN = anchor.BN;
const assert = require("assert");

const GLOBAL_AUTHORITY_SEED = "global-authority";
const POOL_WALLET_SEED = "pool-wallet";
const USER_POOL_SIZE = 2464;

describe('staking_program', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.StakingProgram as Program<StakingProgram>;
  const superOwner = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();
  const lotteryPool = anchor.web3.Keypair.generate();
  const fixedPool = anchor.web3.Keypair.generate();
  const REWARD_TOKEN_MINT = new PublicKey("8EoML7gaBJsgJtepm25wq3GuUCqLYHBoqd3HP1JxtyBx");


  const USER_POOL_SIZE = 2464;
  const GLOBAL_POOL_SIZE = 360_016;

  let nft_token_mint = null;
  let userTokenAccount = null;

  let globalLotteryPoolKey = anchor.web3.Keypair.generate();

  // let rewardAccount = anchor.web3.Keypair.generate();
  // let reward = anchor.web3.Keypair.generate();

  const reward = anchor.web3.Keypair.fromSecretKey(new Uint8Array([154, 43, 74, 184, 192, 57, 192, 123, 59, 172, 107, 58, 107, 47, 129, 73, 187, 15, 160, 217, 13, 135, 47, 181, 246, 63, 94, 26, 245, 108, 183, 36, 107, 138, 196, 135, 102, 88, 153, 43, 141, 165, 202, 167, 48, 225, 231, 113, 123, 61, 176, 248, 90, 204, 240, 109, 165, 204, 141, 5, 100, 184, 81, 99]));

  console.log('Reward Token: ', reward.publicKey.toBase58());
  const rewardToken = new Token(
    provider.connection,
    REWARD_TOKEN_MINT,
    TOKEN_PROGRAM_ID,
    superOwner,
  )


  it('Is initialized!', async () => {
    // Add your test here.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(superOwner.publicKey, 9000000000),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 9000000000),
      "confirmed"
    );

    console.log("super owner =", superOwner.publicKey.toBase58());
    console.log("user =", user.publicKey.toBase58());

    nft_token_mint = await Token.createMint(
      provider.connection,
      user,
      superOwner.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );
    userTokenAccount = await nft_token_mint.createAccount(user.publicKey);

    await nft_token_mint.mintTo(
      userTokenAccount,
      superOwner,
      [],
      1
    );

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_AUTHORITY_SEED)],
      program.programId
    );

    console.log("globalAuthority =", globalAuthority.toBase58());

    const [poolWalletKey, walletBump] = await PublicKey.findProgramAddress(
      [Buffer.from(POOL_WALLET_SEED)],
      program.programId
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(poolWalletKey, 1000000000),
      "confirmed"
    );
    console.log("poolWalletKey =", poolWalletKey.toBase58());

    let ix = SystemProgram.createAccount({
      fromPubkey: superOwner.publicKey,
      newAccountPubkey: globalLotteryPoolKey.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(GLOBAL_POOL_SIZE),
      space: GLOBAL_POOL_SIZE,
      programId: program.programId,
    })

    console.log("globalLotteryPoolKey =", globalLotteryPoolKey.publicKey.toBase58())

    const tx = await program.rpc.initialize(
      bump, {
      accounts: {
        admin: superOwner.publicKey,
        globalAuthority,
        poolWallet: poolWalletKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY
      },
      instructions: [ix],
      signers: [superOwner, globalLotteryPoolKey]
    }
    );

    console.log("Your transaction signature", tx);
  });

  it('initialize fixed pool', async () => {

    let userFixedPoolKey = await PublicKey.createWithSeed(
      user.publicKey,
      "user-fixed-pool",
      program.programId,
    );

    let ix = SystemProgram.createAccountWithSeed({
      fromPubkey: user.publicKey,
      basePubkey: user.publicKey,
      seed: "user-fixed-pool",
      newAccountPubkey: userFixedPoolKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(USER_POOL_SIZE),
      space: USER_POOL_SIZE,
      programId: program.programId,
    });
    console.log("userFixedPool.pubk =", userFixedPoolKey.toBase58())
    const tx = await program.rpc.initializeFixedPool(
      {
        accounts: {
          userFixedPool: userFixedPoolKey,
          owner: user.publicKey
        },
        instructions: [
          ix
        ],
        signers: [user]
      }
    );

    console.log("Your transaction signature", tx);
  })

  it("Stake Nft To Fixed", async () => {

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_AUTHORITY_SEED)],
      program.programId
    );

    console.log("globalAuthority =", globalAuthority.toBase58());

    let userFixedPoolKey = await PublicKey.createWithSeed(
      user.publicKey,
      "user-fixed-pool",
      program.programId,
    );

    /*let destNftTokenAccount = await Token.getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID, 
      TOKEN_PROGRAM_ID,
      nft_token_mint.publicKey,
      user.publicKey
    );*/

    const [staked_nft_address, nft_bump] = await PublicKey.findProgramAddress(
      [Buffer.from("staked-nft"), nft_token_mint.publicKey.toBuffer()],
      program.programId
    );

    //let destNftTokenAccount = await nft_token_mint.createAccount(user.publicKey);
    const rank = 100;
    const tx = await program.rpc.stakeNftToFixed(
      bump, new anchor.BN(rank), {
      accounts: {
        owner: user.publicKey,
        userFixedPool: userFixedPoolKey,
        globalAuthority,
        userNftTokenAccount: userTokenAccount,
        destNftTokenAccount: staked_nft_address,
        nftMint: nft_token_mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY
      },
      signers: [user]
    });

    console.log("Your transaction signature", tx);

    let userFixedPool = await program.account.userPool.fetch(userFixedPoolKey);
    console.log("userFixedPool =", userFixedPool.itemCount.toNumber());
  })

  it("Withdraw Nft From Fixed", async () => {

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_AUTHORITY_SEED)],
      program.programId
    );

    console.log("globalAuthority =", globalAuthority.toBase58());

    const [poolWalletKey, walletBump] = await PublicKey.findProgramAddress(
      [Buffer.from(POOL_WALLET_SEED)],
      program.programId
    );

    console.log("poolWalletKey =", poolWalletKey.toBase58());

    let userFixedPoolKey = await PublicKey.createWithSeed(
      user.publicKey,
      "user-fixed-pool",
      program.programId,
    );

    const [staked_nft_address, nft_bump] = await PublicKey.findProgramAddress(
      [Buffer.from("staked-nft"), nft_token_mint.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.rpc.withdrawNftFromFixed(
      bump, nft_bump, walletBump, {
      accounts: {
        owner: user.publicKey,
        userFixedPool: userFixedPoolKey,
        globalAuthority,
        poolWallet: poolWalletKey,
        userNftTokenAccount: userTokenAccount,
        stakedNftTokenAccount: staked_nft_address,
        nftMint: nft_token_mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY
      },
      signers: [user]
    }
    );

    console.log("Your transaction signature", tx);

    let userFixedPool = await program.account.userPool.fetch(userFixedPoolKey);
    //console.log("userFixedPool =", userFixedPool);
  })

  it("Claim Reward", async () => {

    const [globalAuthority, bump] = await PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_AUTHORITY_SEED)],
      program.programId
    );

    console.log("globalAuthority =", globalAuthority.toBase58());

    const [poolWalletKey, walletBump] = await PublicKey.findProgramAddress(
      [Buffer.from(POOL_WALLET_SEED)],
      program.programId
    );

    console.log("poolWalletKey =", poolWalletKey.toBase58());

    let userFixedPoolKey = await PublicKey.createWithSeed(
      user.publicKey,
      "user-fixed-pool",
      program.programId,
    );

    const [staked_nft_address, nft_bump] = await PublicKey.findProgramAddress(
      [Buffer.from("staked-nft"), nft_token_mint.publicKey.toBuffer()],
      program.programId
    );

    let userRewardAccount = await rewardToken.createAccount(user.publicKey);
    let rewardVault = await rewardToken.createAccount(globalAuthority);

    const tx = await program.rpc.claimReward(
      bump, {
      accounts: {
        owner: user.publicKey,
        userFixedPool: userFixedPoolKey,
        globalAuthority,
        rewardVault: rewardVault,
        userRewardAccount: userRewardAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [user]
    }
    );

    console.log("Your transaction signature", tx);

    let userFixedPool = await program.account.userPool.fetch(userFixedPoolKey);
    //console.log("userFixedPool =", userFixedPool);
  })
});