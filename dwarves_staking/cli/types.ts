import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

export interface GlobalPool {
    lotteryNftCount: anchor.BN,    // 8
    fixedNftCount: anchor.BN,  // 8
}

export interface StakedNFT {
    nftAddr: PublicKey,         // 32
    stakeTime: anchor.BN,   // 8
    rate: anchor.BN,         // 8
}

export interface UserPool {
    // 8 + 2456
    owner: PublicKey,            // 32
    itemCount: anchor.BN,      // 8
    items: StakedNFT[],   // 48 * 50
    rewardTime: anchor.BN,                         // 8
    pendingReward: anchor.BN,                       // 8
}