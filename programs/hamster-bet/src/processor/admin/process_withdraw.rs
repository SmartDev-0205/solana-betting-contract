use {
  anchor_lang::prelude::*,
  anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
  crate::{constant::*, error::ContractError, state::*, utils::*},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawIx {
  race_id: u64,
  amount: u64,
}

#[derive(Accounts)]
#[instruction(ix: WithdrawIx)]
pub struct WithdrawCtx<'info> {
  #[account(mut,
    constraint = is_admin(authority.key) @ ContractError::InvalidAuthority
  )]
  pub authority: Signer<'info>,

  /// CHECK: we read this key only
  pub admin_wallet: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [RACE_TAG, &ix.race_id.to_le_bytes()],
    bump,
  )]
  pub race: Box<Account<'info, Race>>,

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
    token::authority = admin_wallet,
)]
  pub admin_token_vault: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<WithdrawCtx>, ix: WithdrawIx) -> Result<()> {
  require!(
      ctx.accounts.race.get_wining_hamster_id().unwrap() == ctx.accounts.ticket.hamster_id,
      ContractError::InvalidTicket
  );


  let signer_seeds = &[
      RACE_TAG,
      &ix.race_id.to_le_bytes(),
      &[ctx.accounts.race.bump],
  ];
  let signer = &[&signer_seeds[..]];

  anchor_spl::token::transfer(
      CpiContext::new_with_signer(
          ctx.accounts.token_program.to_account_info(),
          Transfer {
              from: ctx.accounts.token_vault.to_account_info(),
              to: ctx.accounts.admin_token_vault.to_account_info(),
              authority: ctx.accounts.race.to_account_info(),
          },
          signer,
      ),
      ix.amount,
  )?;

  Ok(())

}
