use anchor_lang::prelude::*;

declare_id!("43kASWhcDyB7hEhsn5Gpeoao5UFkh5JTh4egSJ39cMaP");

/// constant
pub mod constant;
/// error
pub mod error;
/// processor
pub mod processor;
/// states
pub mod state;

pub mod utils;

use crate::processor::*;

#[program]
pub mod hamster_bet {
    use super::*;

    pub fn create_race(ctx: Context<CreateRaceCtx>, ix: CreateRaceIx) -> Result<()> {
        process_create_race::handler(ctx, ix)
    }

    pub fn add_hamster(ctx: Context<AddHamsterCtx>, ix: AddHamsterIx) -> Result<()> {
        process_add_hamster::handler(ctx, ix)
    }

    pub fn cashout(ctx: Context<CashoutCtx>, ix: CashoutIx) -> Result<()> {
        process_cashout::handler(ctx, ix)
    }

    pub fn withdraw(ctx: Context<WithdrawCtx>, ix: WithdrawIx) -> Result<()> {
        process_withdraw::handler(ctx, ix)
    }

    pub fn update_race(ctx: Context<UpdateRaceCtx>, ix: UpdateRaceIx) -> Result<()> {
        process_update_race::handler(ctx, ix)
    }

    pub fn place_bet(ctx: Context<PlaceBetCtx>, ix: PlaceBetIx) -> Result<()> {
        process_place_bet::handler(ctx, ix)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
