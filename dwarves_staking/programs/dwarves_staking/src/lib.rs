use anchor_lang::{accounts::cpi_account::CpiAccount, prelude::*, AnchorDeserialize};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Token, TokenAccount, Transfer},
};

pub mod account;
pub mod constants;
pub mod error;

use account::*;
use constants::*;
use error::*;

declare_id!("Gz9boUKpghd9Kb1yLcM6uLCa4tHEtGz9ETd3RdUXEpVL");

#[program]
pub mod staking_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, global_bump: u8) -> ProgramResult {
        Ok(())
    }

    pub fn initialize_fixed_pool(ctx: Context<InitializeFixedPool>) -> ProgramResult {
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_init()?;
        fixed_pool.owner = ctx.accounts.owner.key();
        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn stake_nft_to_fixed(
        ctx: Context<StakeNftToFixed>,
        global_bump: u8,
        dwarf: String,
        occupation: String,
        season: String,
        gemstone: String,
    ) -> ProgramResult {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut rate: i64 = 1;
        match dwarf.as_str() {
            DLORD => rate *= 230,
            EULORD => rate *= 220,
            IARCH => rate *= 200,
            INVI => rate *= 200,
            FULORD => rate *= 220,
            LULORD => rate *= 220,
            RULORD => rate *= 220,
            HELLBEND => rate *= 200,
            AIRBEND => rate *= 200,
            PALPATINE => rate *= 200,
            AVATAR => rate *= 200,
            JACKSKELL => rate *= 200,
            ENGLAND => rate *= 200,
            MOUSE => rate *= 200,
            PEPE => rate *= 200,
            REANIMFIEND => rate *= 150,
            CADAVEROUS => rate *= 150,
            REANIMWRAITH => rate *= 140,
            POSSESSWRAITH => rate *= 140,
            ELECTIFIED => rate *= 135,
            ELECTIWRAITH => rate *= 135,
            POSSESSFIED => rate *= 130,
            CADAVEROUSFIED => rate *= 130,
            BURNACCURSED => rate *= 125,
            DEMONIACCURED => rate *= 125,
            MINER => rate *= 142,
            MAULEDACCURSED => rate *= 123,
            FIGHTACCURSED => rate *= 122,
            INFLICTACCURSED => rate *= 122,
            INFERNALACCURSED => rate *= 121,
            CORRUPTEDACCURSED => rate *= 121,
            REANIMATEUNDEAD => rate *= 120,
            DWARFOFWAR => rate *= 135,
            CULTICACCURSED => rate *= 122,
            ELECTRIFIEDUNDEAD => rate *= 120,
            POSSESSEDUNDEAD => rate *= 120,
            FULLMOON => rate *= 133,
            MUSTARDSKULL => rate *= 123,
            CADAVEROUSUN => rate *= 122,
            UMBERSKULL => rate *= 115,
            IVORYSKULL => rate *= 115,
            HALFMOONGR => rate *= 126,
            AUQASKULL => rate *= 110,
            HALFMOON => rate *= 115,
            GAMETIME => rate *= 107,
            BLOODIED => rate *= 107,
            ACCURSED => rate *= 105,
            NEUTRAL => rate *= 100,
            _ => rate *= 100,
        }
        match occupation.as_str() {
            HIGHKING => rate *= 250,
            KING => rate *= 220,
            KINGGUARD => rate *= 100,
            _ => rate *= 100,
        }
        match season.as_str() {
            ONE => rate *= 108,
            TWO => rate *= 107,
            _ => rate *= 100,
        }
        match gemstone.as_str() {
            DIAMOND => rate *= 110,
            LAPISLAZULI => rate *= 106,
            RUBY => rate *= 105,
            OBSIDIAN => rate *= 107,
            EMERALD => rate *= 104,
            FLOURITE => rate *= 103,
            AMETHYST => rate *= 101,
            _ => rate *= 100,
        }

        let staked_item = StakedNFT {
            nft_addr: ctx.accounts.nft_mint.key(),
            stake_time: timestamp,
            rate: rate,
        };
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        fixed_pool.add_nft(staked_item);

        ctx.accounts.global_authority.fixed_nft_count += 1;
        let token_account_info = &mut &ctx.accounts.user_token_account;
        let dest_token_account_info = &mut &ctx.accounts.dest_nft_token_account;
        let token_program = &mut &ctx.accounts.token_program;

        let cpi_accounts = Transfer {
            from: token_account_info.to_account_info().clone(),
            to: dest_token_account_info.to_account_info().clone(),
            authority: ctx.accounts.owner.to_account_info().clone(),
        };
        token::transfer(
            CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
            1,
        )?;
        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn withdraw_nft_from_fixed(
        ctx: Context<WithdrawNftFromFixed>,
        global_bump: u8,
    ) -> ProgramResult {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        let reward: i64 = fixed_pool.remove_nft(
            ctx.accounts.owner.key(),
            ctx.accounts.nft_mint.key(),
            timestamp,
        )?;

        fixed_pool.pending_reward += reward;

        ctx.accounts.global_authority.fixed_nft_count -= 1;

        let token_account_info = &mut &ctx.accounts.user_token_account;
        let dest_token_account_info = &mut &ctx.accounts.dest_nft_token_account;
        let token_program = &mut &ctx.accounts.token_program;
        let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: dest_token_account_info.to_account_info().clone(),
            to: token_account_info.to_account_info().clone(),
            authority: ctx.accounts.global_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(
                token_program.clone().to_account_info(),
                cpi_accounts,
                signer,
            ),
            1,
        )?;
        Ok(())
    }

    #[access_control(user(&ctx.accounts.user_fixed_pool, &ctx.accounts.owner))]
    pub fn claim_reward(ctx: Context<ClaimReward>, global_bump: u8) -> ProgramResult {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut fixed_pool = ctx.accounts.user_fixed_pool.load_mut()?;
        let reward: u64 = fixed_pool.claim_reward(timestamp)? as u64;
        msg!("Reward: {}", reward);
        if ctx.accounts.reward_vault.amount < 1000_000_000 + reward {
            return Err(StakingError::LackLamports.into());
        }
        let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
        let signer = &[&seeds[..]];
        let token_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.user_reward_account.to_account_info(),
            authority: ctx.accounts.global_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer),
            reward,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
        payer = admin
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        constraint = reward_vault.mint == REWARD_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
        constraint = reward_vault.owner == global_authority.key(),
        constraint = reward_vault.amount >= MIN_REWARD_DEPOSIT_AMOUNT,
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeFixedPool<'info> {
    #[account(zero)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct StakeNftToFixed<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
    #[account(
        mut,
        constraint = user_token_account.mint == *nft_mint.to_account_info().key,
        constraint = user_token_account.owner == *owner.key,
        constraint = user_token_account.amount == 1,
    )]
    pub user_token_account: CpiAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = dest_nft_token_account.mint == *nft_mint.to_account_info().key,
        constraint = dest_nft_token_account.owner == *global_authority.to_account_info().key,
    )]
    pub dest_nft_token_account: CpiAccount<'info, TokenAccount>,

    pub nft_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct WithdrawNftFromFixed<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        constraint = user_token_account.mint == *nft_mint.to_account_info().key,
        constraint = user_token_account.owner == *owner.key,
    )]
    pub user_token_account: CpiAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = dest_nft_token_account.mint == *nft_mint.to_account_info().key,
        constraint = dest_nft_token_account.owner == *global_authority.to_account_info().key,
        constraint = dest_nft_token_account.amount == 1,
    )]
    pub dest_nft_token_account: CpiAccount<'info, TokenAccount>,

    pub nft_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub user_fixed_pool: AccountLoader<'info, UserPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(
        mut,
        constraint = reward_vault.mint == REWARD_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
        constraint = reward_vault.owner == global_authority.key(),
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_reward_account.mint == REWARD_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
        constraint = user_reward_account.owner == owner.key(),
    )]
    pub user_reward_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

// Access control modifiers
fn user(pool_loader: &AccountLoader<UserPool>, user: &AccountInfo) -> Result<()> {
    let user_pool = pool_loader.load()?;
    require!(user_pool.owner == *user.key, StakingError::InvalidUserPool);
    Ok(())
}
