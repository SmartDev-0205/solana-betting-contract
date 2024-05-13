use {
  anchor_lang::prelude::*,
  crate::{constant::*, error::ContractError, state::*, utils::*},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateRaceIx {
  race_id: u64,
  status: u8,
  is_end: bool,
  wining_hamster_id: u8,
}

#[derive(Accounts)]
#[instruction(ix: UpdateRaceIx)]
pub struct UpdateRaceCtx<'info> {
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

pub fn handler(ctx: Context<UpdateRaceCtx>, ix: UpdateRaceIx) -> Result<()> {
  let race = &mut ctx.accounts.race;

  race.status = ix.status;

  if ix.is_end {
      ctx.accounts
          .race
          .reveal_wining_hamster(ix.wining_hamster_id);
  }

  Ok(())
}
