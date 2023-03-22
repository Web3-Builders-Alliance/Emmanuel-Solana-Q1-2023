use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        Mint, Token, TokenAccount, Transfer as SplTransfer,
    }
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod binary_options {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        Ok(())
    }

    // deposit fungible SPL tokens
    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        let valid_amount = {
            if amount > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }

        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.deposit_auth.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        anchor_spl::token::transfer(cpi, amount)?;

        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposited_amount = amount;
        //(&mut ctx.accounts.deposit_account).deposited_amount = amount;

        Ok(())
    }

    // withdraw fungible SPL tokens
    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
        let valid_amount = {
            if amount > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }

        let deposit_account = &ctx.accounts.deposit_account;

        // trader must have made a correct prediction and won it
        if !deposit_account.made_prediction && !deposit_account.won_prediction{
            return Err(Errors::InvalidPrediction.into());
        }

        let valid_amount = {
            if amount == deposit_account.total_payout {
                true
            }
            else{false}
        };
        // amount must not exceed total payout amount
        if !valid_amount {
            return Err(Errors::ExceededTotalPayoutAmount.into());
        }

        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_acct.to_account_info(),
            to: ctx.accounts.to_token_acct.to_account_info(),
            authority: ctx.accounts.pda_auth.to_account_info(),
        };

        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_spl::token::transfer(cpi, amount)?;

        Ok(())
    }

    pub fn make_prediction(ctx: Context<MakePrediction>, strike_price: u64) -> Result<()> {
        let valid_amount = {
            if strike_price > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }

        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.strike_price = strike_price;
        deposit_account.made_prediction = true;

        Ok(())
    }

    pub fn process_prediction(ctx: Context<ProcessPrediction>, asset_current_price: u64, winning_amount: u64) -> Result<()> {
        let valid_amount = {
            if asset_current_price > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }

        let valid_amount = {
            if winning_amount > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }

        let deposit_account = &mut ctx.accounts.deposit_account;

        // Trader wins when strike_price is equal to asset_current_price
        let won_prediction = {
            if deposit_account.strike_price == asset_current_price {
                true
            }
            else{false}
        };

        if won_prediction {
            let deposited_amount = deposit_account.deposited_amount;

            let valid_amount = {
                if deposited_amount > winning_amount {
                    true
                }
                else{false}
            };
            // winning Amount exceeds deposited amount
            if !valid_amount {
                return Err(Errors::InvalidWinningAmount.into());
            }

            deposit_account.total_payout = deposited_amount + winning_amount;    
        }

        deposit_account.won_prediction = won_prediction;

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
pub struct DepositSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        payer = deposit_auth,
        associated_token::authority = pda_auth,
    )]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut)]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    #[account(mut)]
    pub to_token_acct: Account<'info, TokenAccount>,
    #[account(mut,
        associated_token::mint = token_mint,
        associated_token::authority = pda_auth,
    )]
    pub from_token_acct: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakePrediction<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessPrediction<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositBase {
    pub deposit_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
    pub deposited_amount: u64,
    pub strike_price: u64,
    pub made_prediction: bool,
    pub won_prediction: bool,
    pub total_payout: u64,
}

impl DepositBase {
    const LEN: usize = 8 + 32 + 1 + 1 + 1 + 8 + 8 + 1 + 1 + 8;
}

#[error_code]
pub enum Errors {
    #[msg("Insufficient amount to withdraw.")]
    InvalidWithdrawAmount,
    #[msg("Amount must be greater than zero.")]
    AmountNotgreaterThanZero,
    #[msg("Withdrawal amount exceeds total payout amount.")]
    ExceededTotalPayoutAmount,
    #[msg("Trader must make a prediction and win it.")]
    InvalidPrediction,
    #[msg("Winning Amount exceeds deposited amount.")]
    InvalidWinningAmount,
}