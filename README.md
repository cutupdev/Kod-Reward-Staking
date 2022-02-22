# KoD-NFT-Staking
Multiple Factor Rewards Staking program for Kingdom of Dwarves NFT collections

## Install Dependencies
- Install `node` and `yarn`
- Install `ts-node` as global command
- Confirm the solana wallet preparation: `/home/fury/.config/solana/id.json` in test case

## Usage
- Main script source for all functionality is here: `/cli/script.ts`
- Program account types are declared here: `/cli/types.ts`
- Idl to make the JS binding easy is here: `/cli/staking_program.json`

Able to test the script functions working in this way.
- Change commands properly in the main functions of the `script.ts` file to call the other functions
- Confirm the `ANCHOR_WALLET` environment variable of the `ts-node` script in `package.json`
- Run `yarn ts-node`

## Features

### As a Smart Contract Owner
For the first time use, the Smart Contract Owner should `initialize` the Smart Contract for global account allocation.
- `initProject`
 
Recall `initialize` function for update the Threshold values after change the constants properly
- `initProject` 

Maintain the Reward token($KCROWN) vault's balance
- `REWARD_TOKEN_MINT` is the reward token mint (for test).
- `rewardVault` is the reward token account for owner. The owner should have the token's `Mint Authority` or should `Fund` regularly.

This is current test value. Should be revised properly.
- `EPOCH` = 60                                    // A day 
- `REWARD_PER_DAY` = 100_000_000                  // 0.1 $KCROWN 
According to the rank of NFTs, there reward amount will be changed automatically following the below logic.

{
  "Dwarf": {
    "The Dark Lord": 2.3,
    "Emerald Underlord": 2.2,
    "Irish Archer": 2,
    "Invisible": 2,
    "Fluorite Underlord": 2.2,
    "Lapis Underlord": 2.2,
    "Ruby Underlord": 2.2,
    "Hellbender": 2,
    "Airbender": 2,
    "Palpatine": 2,
    "Avatar": 2,
    "Jack Skellington": 2,
    "England": 2,
    "Mouse": 2,
    "Pepe": 2,
    "Reanimated Undead Fiend": 1.5,
    "Cadaverous Undead Wraith": 1.5,
    "Reanimated Undead Wraith": 1.4,
    "Possessed Undead Wraith": 1.4,
    "Electrified Undead Fiend": 1.35,
    "Electrified Undead Wraith": 1.35,
    "Possessed Undead Fiend": 1.3,
    "Cadaverous Undead Fiend": 1.3,
    "Burned Accursed": 1.25,
    "Demonic Accursed": 1.25,
    "Miner": 1.42,
    "Mauled Accursed": 1.23,
    "Fight Club Accursed": 1.22,
    "Inflicted Accursed": 1.22,
    "Infernal Accursed": 1.21,
    "Corrupted Accursed": 1.21,
    "Reanimated Undead": 1.2,
    "Dwarf of War": 1.35,
    "Cultic Accursed": 1.22,
    "Electrified Undead": 1.2,
    "Possessed Undead": 1.2,
    "Full Moon": 1.33,
    "Mustard Skull Accursed": 1.23,
    "Cadaverous Undead": 1.22,
    "Umber Skull Accursed": 1.15,
    "Ivory Skull Accursed": 1.15,
    "Half Moon Grey": 1.26,
    "Aqua Skull Accursed": 1.1,
    "Half Moon": 1.15,
    "Gametime": 1.07,
    "Bloodied": 1.07,
    "Accursed": 1.05,
    "Neutral": 1
  },
  "Occupation": {
    "High King": 2.5,
    "King": 2.2,
    "King's Guards": 1
  },
  "Season": {
    "1": 1.08,
    "2": 1.07
  },
  "Gemstone": {
    "Diamond": 1.1,
    "Lapis Lazuli": 1.06,
    "Ruby": 1.05,
    "Obsidian": 1.07,
    "Emerald": 1.04,
    "Flourite": 1.03,
    "Amethyst": 1.01
  },
}

### As a NFT Holder
Stake Shred Collection NFTs with NFT `mint address` and a boolean parameter weather the NFT is Legendary NFT.
- `stakeNft`

### As a Staker
Unstake their staked NFTs with `mint address` and get rewards. ( Calculate generated reward by this NFT too )
- `withdrawNft`

Claim reward to receive generated $KCROWN from their staking.
- `claimReward`
