use anchor_lang::prelude::*;
use deposit::DepositBase;
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<AddtoWhiteList>) -> Result<()> {
        Ok(())
    }

    pub fn is_whitelisted(ctx: Context<IsWhitelisted>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = signer, space = size_of::<WhitelistBase>() )]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddtoWhiteList<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, WhitelistBase>,
    #[account(mut)]
    pub whitelist_account: Account<'info>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref(), whitelist_account.key().as_ref()], bump = deposit_account.auth_bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IsWhitelisted<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, WhitelistBase>,
    #[account(mut, has_one = deposit_auth)]
    pub whitelist_account: Account<'info>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref(), whitelist_account.key().as_ref()], bump = deposit_account.auth_bump)]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WhitelistBase {
    pub deposit_auth: Pubkey,
    pub whitelist_auth: Pubkey,
    pub auth_bump: u8,
}