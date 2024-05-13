use {
    crate::{constant::*, state::*, error::ContractError, utils::*},
    anchor_lang::prelude::*,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddHamsterIx {
    race_id: u64,
    hamster_id: u8,
    boost: u64, // Add boost field
}

#[derive(Accounts)]
#[instruction(ix: AddHamsterIx)]
pub struct AddHamsterCtx<'info> {
    #[account(mut,
        constraint = is_admin(authority.key) @ ContractError::InvalidAuthority
      )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [RACE_TAG, &ix.race_id.to_le_bytes()],
        bump,
    )]
    pub race: Box<Account<'info, Race>>,
}

pub fn handler(ctx: Context<AddHamsterCtx>, ix: AddHamsterIx) -> Result<()> {
    let race = &mut ctx.accounts.race;

    race.hamsters.push(Hamster {
        id: ix.hamster_id,
        pool: 0,
        is_winner: false,
        boost: ix.boost, // Set the boost value
    });

    Ok(())
}
