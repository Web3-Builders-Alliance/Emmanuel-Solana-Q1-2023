use anchor_lang::{prelude::*, system_program};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// betting description length
const DESCRIPTION_LENGTH: usize = 1024;

#[program]
pub mod binary_options {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.admin_deposit_account;
        let admin_auth = &ctx.accounts.admin_auth;

        deposit_account.admin_auth = *ctx.accounts.admin_auth.key;
        deposit_account.admin_auth_bump = *ctx.bumps.get("admin_pda_auth").unwrap();
        deposit_account.admin_sol_vault_bump = ctx.bumps.get("admin_sol_vault").copied();
        deposit_account.is_initialized = true;

        Ok(())
    }

    pub fn create_binary_options(ctx: Context<CreateBinaryOptions>, bet_description: String, bet_amount: u64, strike_price: u64, taker_amount: u64, participantPosition: ParticipantPosition) -> Result<()> {
        if bet_description.trim().is_empty() {
            return Err(Errors::CannotCreateBetting.into());
        }
        if bet_description.as_bytes().len() > DESCRIPTION_LENGTH {
            return Err(Errors::ExceededDescriptionMaxLength.into());
        }

        // bet_amount
        let valid_amount = {
            if bet_amount > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        
        // strike_price
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

        // taker_amount
        let valid_amount = {
            if taker_amount > 0 {
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

        deposit_account.deposit_auth = *ctx.accounts.deposit_auth.key;
        deposit_account.taker_auth = *ctx.accounts.deposit_auth.key;
        deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();
        deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();
        deposit_account.bet_description = bet_description;
        deposit_account.bet_amount = bet_amount;
        deposit_account.strike_price = strike_price;
        deposit_account.taker_amount = taker_amount;
        deposit_account.first_participant = participantPosition;
        deposit_account.betting_state = 1;

        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, bet_amount)?;

        Ok(())
    }

    //  accept binary options and deposit native sol
    pub fn accept_binary_options(ctx: Context<AcceptBinaryOptions>, amount: u64, participant_position: ParticipantPosition) -> Result<()> {
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
        /*
        let valid_amount = {
            if fees > 0 {
                true
            }
            else{false}
        };
        // amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        */
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let sys_program = &ctx.accounts.system_program;

        let valid_amount = {
            if amount == deposit_account.taker_amount {
                true
            }
            else{false}
        };
        // amount must be equal to taker_amount
        if !valid_amount {
            return Err(Errors::InvalidDepositAmount.into());
        }

        // first participant is not allowed to make prediction since they had previously done so in create options.
        if deposit_account.deposit_auth.eq(deposit_auth.key) {
            return Err(Errors::PredictionDisAllowed.into());
        }

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

        // Lets indicate that prediction has been completed by two participants
        deposit_account.made_prediction = true;
        deposit_account.second_participant = participant_position;

        // Lets maintain the pubkey of the second participant
        deposit_account.taker_auth = *ctx.accounts.deposit_auth.key;
        // Lets change the betting state to indicate limit of two participants has been met
        deposit_account.betting_state = 2;

        //deposit_account.sol_vault_bump = ctx.bumps.get("sol_vault").copied();

        /*
        // step 1: deposit sol to admin vault
        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.admin_sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, fees)?;
        */

        // step 1: deposit sol to participants(limited to two) vault
        let cpi_accounts = system_program::Transfer {
            from: deposit_auth.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

    // withdraw native sol 
    pub fn withdraw_participant_funds(ctx: Context<WithdrawParticipantFunds>, amount: u64) -> Result<()> {
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
        let deposit_auth = &ctx.accounts.deposit_auth;

        // we only allow either first or second participants to withdraw since the two made the prediction
        if deposit_account.deposit_auth.eq(deposit_auth.key) || deposit_account.taker_auth.eq(deposit_auth.key) {
            return Err(Errors::WithdrawalDisAllowed.into());
        }

        // participant must have made a correct prediction and won it
        if !deposit_account.made_prediction{
            return Err(Errors::InvalidPrediction.into());
        }
        /*
        let winning_participant = deposit_account.won_prediction;

        let first_participant = {
            match winning_participant {
                Participants::First => true,
                Participants::Second => false,
                _ => false
            }
        };

        let second_participant = {
            match winning_participant {
                Participants::First => false,
                Participants::Second => true,
                _ => false
            }
        };


        let valid_participant_winner = {
            if !first_participant_position && deposit_account.deposit_auth.eq(deposit_auth.key) {
                true
            }
            else if second_participant_position && deposit_account.taker_auth.eq(deposit_auth.key) {
                true
            }
            else{false}
        };
        */

        let valid_participant_winner = {
            if deposit_account.winner_auth.eq(deposit_auth.key) {
                true
            }
            else{false}
        };

        if !valid_participant_winner {
            // Invalid participant winner.
            return Err(Errors::InvalidWinner.into());
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
        //let deposit_account = &ctx.accounts.deposit_account;
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
    /*
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
    */
    pub fn process_prediction(ctx: Context<ProcessPrediction>, winning_position: ParticipantPosition, bet_fees: u64) -> Result<()> {
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
            if bet_fees > 0 {
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

        // participant wins when strike_price is equal to asset_current_price
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
        let winning_position_bool = {
            match winning_position {
                ParticipantPosition::Long => true,
                ParticipantPosition::Short => false,
                _ => false
            }
        };
        /*
        let winning_participant = {
            if first_participant_position && winning_position_bool {
                Participants::First
            }
            else if second_participant_position && winning_position_bool {
                Participants::Second
            }
            else{Participants::Unknown}
        };
        deposit_account.won_prediction = winning_participant;
        */

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

        let bet_amount = deposit_account.bet_amount;
        let taker_amount = deposit_account.taker_amount;

        let valid_amount = {
            if bet_amount + taker_amount > bet_fees  {
                true
            }
            else{false}
        };
        // bet_fees exceeds (bet_amount + taker_amount)
        if !valid_amount {
            return Err(Errors::InvalidWinningAmount.into());
        }
        
        let total_payout: u64 = bet_amount + taker_amount - bet_fees;
        let mut valid_position = false;
        // first_participant - deposit_account.deposit_auth
        // second_participant -  deposit_account.taker_auth
        if first_participant_position && winning_position_bool {
            deposit_account.winner_auth = deposit_account.deposit_auth;
            deposit_account.total_payout = total_payout;
            valid_position = true;
        }
        else if second_participant_position && winning_position_bool {
            deposit_account.winner_auth = deposit_account.taker_auth;
            deposit_account.total_payout = total_payout;
            valid_position = true;
        }
        else{}

        if valid_position {
            // step 1: deposit (bet_fees) sol to admin vault
            let cpi_accounts = system_program::Transfer {
                from: deposit_auth.to_account_info(),
                to: ctx.accounts.admin_sol_vault.to_account_info(),
            };

            let cpi = CpiContext::new(sys_program.to_account_info(), cpi_accounts);

            system_program::transfer(cpi, bet_fees)?;
        }

        Ok(())
    }

    // admin (on behalf of house) withdraws native sol 
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let sys_program = &ctx.accounts.system_program;
        let deposit_account = &ctx.accounts.admin_deposit_account;
        let pda_auth = &mut ctx.accounts.admin_pda_auth;
        let sol_vault = &mut ctx.accounts.admin_sol_vault;

        let cpi_accounts = system_program::Transfer {
            from: sol_vault.to_account_info(),
            to: ctx.accounts.admin_auth.to_account_info(),
        };

        let seeds = &[
            b"admin_sol_vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.admin_sol_vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(sys_program.to_account_info(), cpi_accounts, signer);

        system_program::transfer(cpi, amount)?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin_auth, space = DepositBaseAdmin::LEN,
        constraint = !admin_deposit_account.is_initialized @ Errors::AccountAlreadyInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBaseAdmin>,
    #[account(seeds = [b"admin_auth", admin_deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin_sol_vault", admin_pda_auth.key().as_ref()], bump)]
    pub admin_sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub admin_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateBinaryOptions<'info> {
    #[account(init, payer = deposit_auth, space = BinaryOption::LEN)]
    pub deposit_account: Account<'info, BinaryOption>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump)]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ Errors::AccountNotInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBaseAdmin>,
    //
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptBinaryOptions<'info> {
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ Errors::AccountNotInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBaseAdmin>,
    #[account(seeds = [b"admin_auth", admin_deposit_account.key().as_ref()], bump = admin_deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin_sol_vault", admin_pda_auth.key().as_ref()], bump = admin_deposit_account.admin_sol_vault_bump.unwrap())]
    pub admin_sol_vault: SystemAccount<'info>,
    //admin accs
    #[account(mut,
        constraint = deposit_account.betting_state == 1 @ Errors::InvalidParticipantsLimit,
    )]
    pub deposit_account: Account<'info, BinaryOption>,
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
pub struct WithdrawParticipantFunds<'info> {
    pub deposit_account: Account<'info, BinaryOption>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"sol_vault", pda_auth.key().as_ref()], bump = deposit_account.sol_vault_bump.unwrap())]
    pub sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}
/*
#[derive(Accounts)]
pub struct MakePrediction<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, DepositBase>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}
*/
#[derive(Accounts)]
pub struct ProcessPrediction<'info> {
    #[account(has_one = deposit_auth)]
    pub deposit_account: Account<'info, BinaryOption>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    //admin accs
    #[account(mut,
        constraint = admin_deposit_account.is_initialized @ Errors::AccountNotInitialized
    )]
    pub admin_deposit_account: Account<'info, DepositBaseAdmin>,
    #[account(seeds = [b"admin_auth", admin_deposit_account.key().as_ref()], bump = admin_deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin_sol_vault", admin_pda_auth.key().as_ref()], bump = admin_deposit_account.admin_sol_vault_bump.unwrap())]
    pub admin_sol_vault: SystemAccount<'info>,
    //admin accs
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(has_one = admin_auth)]
    pub admin_deposit_account: Account<'info, DepositBaseAdmin>,
    #[account(seeds = [b"admin_auth", admin_deposit_account.key().as_ref()], bump = admin_deposit_account.admin_auth_bump)]
    /// CHECK: no need to check this.
    pub admin_pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"admin_sol_vault", admin_pda_auth.key().as_ref()], bump = admin_deposit_account.admin_sol_vault_bump.unwrap())]
    pub admin_sol_vault: SystemAccount<'info>,
    #[account(mut)]
    pub admin_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BinaryOption {
    pub deposit_auth: Pubkey,
    pub taker_auth: Pubkey,
    pub winner_auth: Pubkey,
    pub auth_bump: u8,
    pub sol_vault_bump: Option<u8>,
    pub bet_description: String,
    pub bet_amount: u64,
    pub taker_amount: u64,
    pub strike_price: u64,
    pub deposited_amount: u64,
    pub made_prediction: bool,
    //pub won_prediction: Participants,
    pub total_payout: u64,
    pub first_participant: ParticipantPosition,
    pub second_participant: ParticipantPosition,
    pub betting_state: u8,
}

impl BinaryOption {
    const LEN: usize = 8 + 32 + 32 + 1 + 1 + 1 + 8 + 8 + 1 + 1 + 8 + 4 + DESCRIPTION_LENGTH + 1 + 1 + 1;
}
#[account]
pub struct DepositBaseAdmin {
    pub admin_auth: Pubkey,
    pub admin_auth_bump: u8,
    pub admin_sol_vault_bump: Option<u8>,
    pub is_initialized: bool,
}

impl DepositBaseAdmin {
    const LEN: usize = 8 + 32 + 1 + 1 + 1 + 1;
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
    #[msg("Deposit amount must be equal to bet_amount.")]
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
    #[msg("Single participant is not allowed to take both predictions.")]
    PredictionDisAllowed,
    #[msg("Participant not allowed to make a withdrawal.")]
    WithdrawalDisAllowed,
    #[msg("Invalid participant winner.")]
    InvalidWinner,
    #[msg("Create options not initialised or participants limit of two cannot be exceeded.")]
    InvalidParticipantsLimit,
    #[msg("Account is not initialized.")]
    AccountNotInitialized,
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,
}