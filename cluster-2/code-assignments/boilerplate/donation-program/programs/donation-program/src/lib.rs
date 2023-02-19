use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Hs2XLUNyzuQPdMwuA3izum57DsAZLRANsbXFW2mZ3ozk");

// campaign name text length
const NAME_LENGTH: usize = 100;
// campaign description length
const DESCRIPTION_LENGTH: usize = 1024;

#[program]
pub mod donation_program {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String, target_amount: u64) -> Result<()> {
        if name.trim().is_empty() || description.trim().is_empty() {
          return Err(Errors::CannotCreateCampaign.into());
        }
        if name.as_bytes().len() > NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        if description.as_bytes().len() > DESCRIPTION_LENGTH {
            return Err(Errors::ExceededDescriptionMaxLength.into());
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
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.target_amount = target_amount;
        // * - means dereferencing
        campaign.owner = *ctx.accounts.user.key;
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
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        if campaign.owner != *user.key {
            return Err(Errors::InvalidOwner.into());
        }
        // Rent balance depends on data size
        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(Errors::InvalidWithdrawAmount.into());
        }
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    } 

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let valid_amount = {
          if amount > 0 {
              true
          }
            else{false}
        };
        //  donation amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        //  donation target amount cannot be exceeded
        let target_amount = &ctx.accounts.campaign.target_amount;
        let total_amount_donated  = &ctx.accounts.campaign.amount_donated;
        if *total_amount_donated + amount > *target_amount {
            return Err(Errors::ExceededTargetAmount.into());
        }
        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
            ]
        );
        let campaign = &mut ctx.accounts.campaign;
        campaign.amount_donated += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    // init means to create campaign account
    // bump to use unique address for campaign account
    #[account(init, payer=user, space=size_of::<Campaign>() + NAME_LENGTH + DESCRIPTION_LENGTH, seeds=[b"campaign".as_ref(), user.key().as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Campaign {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
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
    #[msg("Donation target amount Exceeded.")]
    ExceededTargetAmount,
    #[msg("Campaign cannot be created, missing data")]
    CannotCreateCampaign,
    #[msg("Exceeded campaign name max length")]
    ExceededNameMaxLength,
    #[msg("Exceeded campaign description max length")]
    ExceededDescriptionMaxLength,
}