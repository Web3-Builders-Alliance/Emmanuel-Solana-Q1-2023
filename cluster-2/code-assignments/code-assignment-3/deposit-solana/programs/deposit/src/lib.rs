use anchor_lang::{prelude::*, system_program};
/*
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer as SplTransfer},
};
*/
use anchor_spl::{associated_token::AssociatedToken};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer};


declare_id!("7YKyo13HtdB823RiWHacDR74wc7VeU8vkMZGJDP2nSUB");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    pub fn initialize_spl(ctx: Context<InitializeSpl>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    //methods for depositing native tokens
    pub fn deposit_native(ctx: Context<DepositNative>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let sys_program = &ctx.accounts.system_program;

        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();
        
        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    //methods for depositing Spl tokens
    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let sys_program = &ctx.accounts.system_program;

        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

        // Transfer tokens from deposit_auth to sol_vault
        token::transfer(
            ctx.accounts.into_transfer_to_sol_vault_context(),
            amount)?;

        Ok(())
    }

    //methods for withdrawing native tokens
    pub fn withdraw_native(ctx: Context<WithdrawNative>, amount: u64) -> Result<()> {
        let sys_program = &ctx.accounts.system_program;
        let deposit_account = &ctx.accounts.deposit_account;
        let pda_auth = &mut ctx.accounts.pda_auth;
        let sol_vault = &mut ctx.accounts.sol_vault;

        let cpi_accounts = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.deposit_auth.to_account_info(),
        };

        let seeds = &[
            b"sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.sol_vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    //methods for withdrawing Spl tokens
    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        let deposit_account = &ctx.accounts.deposit_account;
        let pda_auth = &mut ctx.accounts.pda_auth;
        //let sol_vault = &mut ctx.accounts.sol_vault;

        let seeds = &[
            b"sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.sol_vault_bump.unwrap()],
        ];

        // Transfer tokens from sol_vault to deposit_auth
        token::transfer(
            ctx.accounts.into_transfer_to_deposit_auth_context().with_signer(&[&seeds[..]]),
            amount)?;

        Ok(())
    }
   

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = deposit_auth, space = DepositBase::LEN)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositNative<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawNative<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeSpl<'info> {
    #[account(init, payer = deposit_auth, space = DepositBase::LEN)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    /*
    #[account(mut)]
    pub sol_vault_token_account: Account<'info, TokenAccount>,
    #[account(
        constraint = *sol_vault.to_account_info().owner == *sol_vault.to_account_info().key @ ProgramError::IncorrectProgramId
    )]
    */
    #[account(
        init_if_needed,
        payer = deposit_auth, 
        associated_token::mint = mint, 
        associated_token::authority = deposit_auth
    )]
    pub sol_vault_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    //
    #[account(mut)]
    pub deposit_auth_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    //#[account(has_one = deposit_auth)]
    pub sol_vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub deposit_auth_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLimit<'info> {
    #[account(init, payer = deposit_auth, space = size_of::<Limit>())]
    pub deposit_account: Account<'info, Limit>,
    #[account(seeds = [b"limit", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    //pub mint_program: Program<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveLimit<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, Limit>,
    #[account(seeds = [b"limit", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    //#[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    //pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateLimit<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, Limit>,
    #[account(seeds = [b"limit", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    //#[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    //pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptLimit<'info> {
    #[account(mut, has_one = deposit_auth)]
    pub deposit_account: Account<'info, Limit>,
    #[account(seeds = [b"limit", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    //#[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    //pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositBase {
    pub deposit_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
}

#[account]
pub struct Limit {
    pub asset_holding_pda: Option<Pubkey>,
    pub asset: Asset,
    pub ask_price_per_asset: u64,
    pub ask_asset: Asset,
    pub ask_asset_pda: Option<Pubkey>,
}

#[account]
pub struct Asset {
    pub asset_type: String,
    pub asset_metadata: Option<Pubkey>,
    pub asset_mint: Option<Pubkey>,
}

impl DepositBase {
    const LEN: usize = 8 + 32 + 1 + 1 + 1;
}

impl<'info> DepositSpl<'info> {
    fn into_transfer_to_sol_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, SplTransfer<'info>> {
        let cpi_accounts = SplTransfer {
            from: self.deposit_auth_token_account.to_account_info().clone(),
            to: self.sol_vault_token_account.to_account_info().clone(),
            authority: self.deposit_auth.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> WithdrawSpl<'info> {
    fn into_transfer_to_deposit_auth_context(&self) -> CpiContext<'_, '_, '_, 'info, SplTransfer<'info>> {
        let cpi_accounts = SplTransfer {
            from: self.sol_vault_token_account.to_account_info().clone(),
            to: self.deposit_auth_token_account.to_account_info().clone(),
            authority: self.sol_vault.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}