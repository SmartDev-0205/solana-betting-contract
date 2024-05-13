use {
  anchor_lang::prelude::*,
  anchor_spl::token::{Mint, Token, TokenAccount},
  crate::{constant::*, error::ContractError, state::*, utils::*},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateRaceIx {
  race_id: u64,
}

#[derive(Accounts)]
#[instruction(ix: CreateRaceIx)]
pub struct CreateRaceCtx<'info> {
  #[account(mut,
    constraint = is_admin(authority.key) @ ContractError::InvalidAuthority
  )]
  pub authority: Signer<'info>,

  #[account(
    init,
    seeds = [RACE_TAG, &ix.race_id.to_le_bytes()],
    bump,
    payer = authority,
    space = std::mem::size_of::<Race>() + 8 + 10 * std::mem::size_of::<Hamster>()
  )]
  pub race: Box<Account<'info, Race>>,

  #[account()]
  /// CHECK: we read this key only
  pub token_mint: Account<'info, Mint>,

  #[account(
    init,
    token::mint = token_mint,
    token::authority = race,
    seeds = [ TOKEN_VAULT_TAG, race.key().as_ref(), token_mint.key().as_ref()],
    bump,
    payer = authority,
  )]
  pub token_vault: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CreateRaceCtx>, ix: CreateRaceIx) -> Result<()> {
  let race = &mut ctx.accounts.race;

  race.bump = ctx.bumps.race;
  race.authority = ctx.accounts.authority.key();
  race.token_mint = ctx.accounts.token_mint.key();
  race.hamsters = Vec::new();
  race.status = 0;
  race.race_pool = 0;
  race.house_pool = 0;
  race.boost_pool = 0;
  race.race_id = ix.race_id;
  race.created_at = ctx.accounts.clock.unix_timestamp as u64;

  Ok(())
}
