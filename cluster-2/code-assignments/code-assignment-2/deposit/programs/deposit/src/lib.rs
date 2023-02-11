use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// deposit name text length
const NAME_LENGTH: usize = 100;

#[program]
pub mod deposit {
    use super::*;

    const ZERO_AMOUNT: u64 = 0;

    pub fn initialize(ctx: Context<Initialize>, name: String, target_amount: u64) -> Result<()> {
        if name.trim().is_empty() {
          return Err(Errors::CannotCreateCampaign.into());
        }
        if name.as_bytes().len() > NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        let valid_amount = {
          if target_amount > 0 {
              true
          }
            else{false}
        };
        //  target amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        let deposit_state = &mut ctx.accounts.deposit_state;
        deposit_state.name = name;
        deposit_state.amount_deposited = ZERO_AMOUNT;
        deposit_state.target_amount = target_amount;
        // * - means dereferencing
        deposit_state.owner = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let valid_amount = {
          if amount > 0 {
              true
          }
            else{false}
        };
        //  deposit amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        //  deposit target amount cannot be exceeded
        let target_amount = &ctx.accounts.deposit_state.target_amount;
        let total_amount_donated  = &ctx.accounts.deposit_state.amount_deposited;
        if *total_amount_donated + amount > *target_amount {
            return Err(Errors::ExceededTargetAmount.into());
        }
        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.deposit_state.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.deposit_state.to_account_info(),
            ]
        );
        let deposit_state = &mut ctx.accounts.deposit_state;
        deposit_state.amount_deposited += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let valid_amount = {
          if amount > 0 {
              true
          }
            else{false}
        };
        //  withdrawal amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        let deposit_state = &mut ctx.accounts.deposit_state;
        let user = &mut ctx.accounts.user;
        if deposit_state.owner != *user.key {
            return Err(Errors::InvalidOwner.into());
        }
        // Rent balance depends on data size
        let rent_balance = Rent::get()?.minimum_balance(deposit_state.to_account_info().data_len());
        if **deposit_state.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(Errors::InvalidWithdrawAmount.into());
        }
        **deposit_state.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // init means to create deposit account
    // bump to use unique address for deposit account
    #[account(init, payer=user, space=size_of::<DepositState>() + NAME_LENGTH, seeds=[b"deposit-state".as_ref(), user.key().as_ref()], bump)]
    pub deposit_state: Account<'info, DepositState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub deposit_state: Account<'info, DepositState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub deposit_state: Account<'info, DepositState>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct DepositState {
    pub owner: Pubkey,
    pub name: String,
    pub amount_deposited: u64,
    pub target_amount: u64,
}

#[error_code]
pub enum Errors {
    #[msg("The user is not the owner of the campaign.")]
    InvalidOwner,
    #[msg("Insufficient amount to withdraw.")]
    InvalidWithdrawAmount,
    #[msg("Amount must be greater than zero.")]
    AmountNotgreaterThanZero,
    #[msg("Deposit target amount Exceeded.")]
    ExceededTargetAmount,
    #[msg("Deposit cannot be created, missing data")]
    CannotCreateCampaign,
    #[msg("Exceeded name max length")]
    ExceededNameMaxLength,
}