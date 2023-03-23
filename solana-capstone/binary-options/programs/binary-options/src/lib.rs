use anchor_lang::{prelude::*, system_program};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// betting description length
const DESCRIPTION_LENGTH: usize = 1024;

#[program]
pub mod binary_options {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, betting_description: String, amount: u64, participantPosition: ParticipantPosition) -> Result<()> {
        if betting_description.trim().is_empty() {
            return Err(Errors::CannotCreateBetting.into());
        }
        if betting_description.as_bytes().len() > DESCRIPTION_LENGTH {
            return Err(Errors::ExceededDescriptionMaxLength.into());
        }
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
        
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        deposit_account.betting_description = betting_description;
        deposit_account.betting_amount = amount;
        deposit_account.first_participant = participantPosition;
        Ok(())
    }

    // deposit native sol
    pub fn deposit_native(ctx: Context<DepositNative>, amount: u64) -> Result<()> {
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

        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let sys_program = &ctx.accounts.system_program;

        let valid_amount = {
            if amount == deposit_account.betting_amount {
                true
            }
            else{false}
        };
        // amount must be equal to betting_amount
        if !valid_amount {
            return Err(Errors::InvalidDepositAmount.into());
        }

        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    // withdraw native sol 
    pub fn withdraw_native(ctx: Context<WithdrawNative>, amount: u64) -> Result<()> {
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
        if !deposit_account.made_prediction{
            return Err(Errors::InvalidPrediction.into());
        }

        // make a check to determine the person withdrawing is the one who won the prediction
        // Save the public key of the one who won and use it here !!!
        /*
        if !deposit_account.made_prediction && !deposit_account.won_prediction{
            return Err(Errors::InvalidPrediction.into());
        }
        */
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

    pub fn make_prediction(ctx: Context<MakePrediction>, strike_price: u64, participant_position: ParticipantPosition) -> Result<()> {
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
        /*
        if deposit_account.first_participant == participantPosition {
            // Both predictions cannot not be same.
            return Err(Errors::PredictionCannotBeSame.into()); 
        }
        */
        let first_participant_position = {
            match deposit_account.first_participant {
                ParticipantPosition::Long => true,
                ParticipantPosition::Short => false,
                _ => false
            }
        };
        let second_participant_position = {
            match participant_position {
                ParticipantPosition::Long => true,
                ParticipantPosition::Short => false,
                _ => false
            }
        };

        let valid_participant_position = {
            if !first_participant_position && second_participant_position {
                true
            }
            else if first_participant_position && !second_participant_position {
                true
            }
            else{false}
        };

        if !valid_participant_position {
            // Both predictions cannot not be same.
            return Err(Errors::PredictionCannotBeSame.into()); 
        }

        //deposit_account.strike_price = strike_price;
        deposit_account.made_prediction = true;
        deposit_account.second_participant = participant_position;

        Ok(())
    }

    pub fn process_prediction(ctx: Context<ProcessPrediction>, winning_position: ParticipantPosition, winning_amount: u64) -> Result<()> {
        /*
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
        */
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
        /*
        let won_prediction = {
            if deposit_account.strike_price == asset_current_price {
                true
            }
            else{false}
        };
        */
        let first_participant_position = {
            match deposit_account.first_participant {
                ParticipantPosition::Long => true,
                ParticipantPosition::Short => false,
                _ => false
            }
        };
        let second_participant_position = {
            match deposit_account.second_participant {
                ParticipantPosition::Long => true,
                ParticipantPosition::Short => false,
                _ => false
            }
        };
        let winning_position = {
            match winning_position {
                ParticipantPosition::Long => true,
                ParticipantPosition::Short => false,
                _ => false
            }
        };

        let winning_participant = {
            if first_participant_position && winning_position {
                Participants::First
            }
            else if second_participant_position && winning_position {
                Participants::Second
            }
            else{Participants::Unknown}
        };
        /*
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
        */
        deposit_account.won_prediction = winning_participant;

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
    pub betting_description: String,
    pub betting_amount: u64,
    pub deposited_amount: u64,
    //pub strike_price: u64,
    pub made_prediction: bool,
    pub won_prediction: Participants,
    pub total_payout: u64,
    pub first_participant: ParticipantPosition,
    pub second_participant: ParticipantPosition,
}

impl DepositBase {
    const LEN: usize = 8 + 32 + 1 + 1 + 1 + 8 + 8 + 1 + 1 + 8 + 4 + DESCRIPTION_LENGTH + 1 + 1 + 1;
}
//Calculate the space for the enum. I just gave it value 1
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum ParticipantPosition {
    Long,
    Short,
}
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum Participants {
    First,
    Second,
    Unknown,
}

#[error_code]
pub enum Errors {
    #[msg("Insufficient amount to withdraw.")]
    InvalidWithdrawAmount,
    #[msg("Amount must be greater than zero.")]
    AmountNotgreaterThanZero,
    #[msg("Withdrawal amount exceeds total payout amount.")]
    ExceededTotalPayoutAmount,
    #[msg("Deposit amount must be equal to betting_amount.")]
    InvalidDepositAmount,
    #[msg("Participant must make a prediction and win it.")]
    InvalidPrediction,
    #[msg("Winning Amount exceeds deposited amount.")]
    InvalidWinningAmount,
    #[msg("Betting cannot be created, missing data")]
    CannotCreateBetting,
    #[msg("Exceeded betting description max length")]
    ExceededDescriptionMaxLength,
    #[msg("Both predictions cannot not be same.")]
    PredictionCannotBeSame,
}