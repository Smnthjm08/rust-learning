use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("C5LLVLEX77U6XQXU6xfh2Woor6wBDXf4TyByZBPKbZWy");

#[program]
pub mod staking_program {

    use super::*;

    pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
        // create new pda account
        let user = &ctx.accounts.user;
        let stake_account = &mut ctx.accounts.stake_account;

        let clock = Clock::get()?;

        stake_account.owner = user.key();
        stake_account.staked_amount = 0;
        stake_account.reward_points = 0;
        stake_account.last_updated = clock.unix_timestamp;
        stake_account.bump = ctx.bumps.stake_account;

        msg!("Stake account created for user: {}", user.key());
        Ok(())
    }

    pub fn stake_amount(ctx: Context<StakeAmount>, amount: u64) -> Result<()> {
        // stake amount
        require!(amount > 0, ErrorCode::InvalidAmount);

        let user = &ctx.accounts.user;

        let stake_account: &mut _ = &mut ctx.accounts.stake_account;

        let clock = Clock::get()?;

        // let time_elapsed: u64 = clock
        //     .unix_timestamp
        //     .checked_sub(stake_account.last_updated)
        //     .ok_or(ErrorCode::MathOverflow)? as u64;

        let time_elapsed_i64 = clock
            .unix_timestamp
            .checked_sub(stake_account.last_updated)
            .ok_or(ErrorCode::MathOverflow)?;

        let time_elapsed: u64 = time_elapsed_i64
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?;

        let additional_rewards = stake_account
            .staked_amount
            .checked_mul(time_elapsed)
            .ok_or(ErrorCode::MathOverflow)?;

        stake_account.reward_points = stake_account
            .reward_points
            .checked_add(additional_rewards)
            .ok_or(ErrorCode::MathOverflow)?;

        stake_account.last_updated = clock.unix_timestamp;

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: user.to_account_info(),
                to: stake_account.to_account_info(),
            },
        );

        system_program::transfer(cpi_context, amount)?;

        stake_account.staked_amount = stake_account
            .staked_amount
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        msg!(
            "User {} staked {} lamports. Total staked: {}",
            user.key(),
            amount,
            stake_account.staked_amount
        );

        Ok(())
    }

    pub fn unstake_amount(ctx: Context<UnstakeAmount>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let user = &ctx.accounts.user;
        let stake_account = &mut ctx.accounts.stake_account;
        let clock = Clock::get()?;

        // -------- Update Rewards --------
        let time_elapsed_i64 = clock
            .unix_timestamp
            .checked_sub(stake_account.last_updated)
            .ok_or(ErrorCode::MathOverflow)?;

        let time_elapsed: u64 = time_elapsed_i64
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?;

        let additional_rewards = stake_account
            .staked_amount
            .checked_mul(time_elapsed)
            .ok_or(ErrorCode::MathOverflow)?;

        stake_account.reward_points = stake_account
            .reward_points
            .checked_add(additional_rewards)
            .ok_or(ErrorCode::MathOverflow)?;

        stake_account.last_updated = clock.unix_timestamp;

        // -------- Ensure Sufficient Balance --------
        require!(
            stake_account.staked_amount >= amount,
            ErrorCode::InsufficientStakedAmount
        );

        let seeds: &[&[u8]] = &[b"seed", some_pubkey.as_ref(), &[bump]];
        let signer_seeds = &[seeds];
        // let seeds = &[b"stake", user.key().as_ref(), &[stake_account.bump]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: stake_account.to_account_info(),
                to: user.to_account_info(),
            },
            signer_seeds,
        );

        system_program::transfer(cpi_context, amount)?;

        stake_account.staked_amount = stake_account
            .staked_amount
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        msg!(
            "User {} unstaked {} lamports. Remaining stake: {}",
            user.key(),
            amount,
            stake_account.staked_amount
        );

        Ok(())
    }
    
}

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub reward_points: u64,
    pub last_updated: i64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateNewAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeAmount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == user.key() @ ErrorCode::Unauthorized
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeAmount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == user.key() @ ErrorCode::Unauthorized
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Only the owner can perform this action")]
    Unauthorized,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Arithmetic overflow or underflow")]
    MathOverflow,

    #[msg("Insufficient staked balance")]
    InsufficientStakedAmount,
}
