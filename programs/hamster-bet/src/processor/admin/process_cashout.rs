use {
  anchor_lang::prelude::*,
  anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
  crate::{constant::*, error::ContractError, state::*, utils::*},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CashoutIx {
  race_id: u64
}

#[derive(Accounts)]
#[instruction(ix: CashoutIx)]
pub struct CashoutCtx<'info> {
  #[account(mut,
    constraint = is_admin(authority.key) @ ContractError::InvalidAuthority
  )]
  pub authority: Signer<'info>,

  /// CHECK: we read this key only
  pub user_wallet: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [RACE_TAG, &ix.race_id.to_le_bytes()],
    bump,
  )]
  pub race: Box<Account<'info, Race>>,

  #[account(
    mut,
    seeds = [TICKET_TAG, race.key().as_ref(), user_wallet.key().as_ref()],
    bump,
    constraint = user_wallet.key() == ticket.authority && ticket.settled == false @ ContractError::InvalidCashout
  )]
  pub ticket: Box<Account<'info, Ticket>>,

  #[account(
    constraint = token_mint.key() == race.token_mint @ ContractError::InvalidToken,
)]
  /// CHECK: we read this key only
  pub token_mint: Account<'info, Mint>,

  #[account(
    mut,
  token::mint = token_mint,
  token::authority = race,
  seeds = [ TOKEN_VAULT_TAG, race.key().as_ref(), token_mint.key().as_ref()],
  bump
)]
  pub token_vault: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    token::mint = token_mint,
    token::authority = user_wallet,
)]
  pub user_token_vault: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CashoutCtx>, ix: CashoutIx) -> Result<()> {
  require!(
      ctx.accounts.race.get_wining_hamster_id().unwrap() == ctx.accounts.ticket.hamster_id,
      ContractError::InvalidTicket
  );

  // Retrieve the winning hamster's pool amount
  let winning_hamster_id = ctx.accounts.race.get_wining_hamster_id().unwrap();
  let winning_hamster_pool = ctx.accounts.race.hamsters[winning_hamster_id as usize].pool;
  let winning_hamster_boost = ctx.accounts.race.hamsters[winning_hamster_id as usize].boost;

  let signer_seeds = &[
      RACE_TAG,
      &ix.race_id.to_le_bytes(),
      &[ctx.accounts.race.bump],
  ];
  let signer = &[&signer_seeds[..]];

  let user_bet_amount = ctx.accounts.ticket.amount as u64;
  let race_pool = (ctx.accounts.race.race_pool / 1_000) as u64;
  let house_pool = (ctx.accounts.race.house_pool / 1_000) as u64;
  let boost_pool = (ctx.accounts.race.boost_pool / 1_000) as u64;

  if winning_hamster_pool != 0 {
      // TODO: calculate the cashout amount.
      let cashout_amount = user_bet_amount
          * (race_pool + boost_pool * winning_hamster_boost / 100)
          / winning_hamster_pool as u64;

      // let cashout_amount = user_bet_amount as u64;
      msg!("cash amount: {}", race_pool);

      anchor_spl::token::transfer(
          CpiContext::new_with_signer(
              ctx.accounts.token_program.to_account_info(),
              Transfer {
                  from: ctx.accounts.token_vault.to_account_info(),
                  to: ctx.accounts.user_token_vault.to_account_info(),
                  authority: ctx.accounts.race.to_account_info(),
              },
              signer,
          ),
          cashout_amount,
      )?;

      ctx.accounts.ticket.settled = true;
      ctx.accounts.ticket.claim = cashout_amount;

      Ok(())
  } else {
      Err(ContractError::InvalidCashout.into())
  }
}
