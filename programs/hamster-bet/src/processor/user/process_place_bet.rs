use {
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
    crate::{constant::*, error::ContractError, state::*},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PlaceBetIx {
    race_id: u64,
    hamster_id: u8,
    amount: u64,
}

#[derive(Accounts)]
#[instruction(ix: PlaceBetIx)]
pub struct PlaceBetCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [RACE_TAG, &ix.race_id.to_le_bytes()],
        bump,
      )]
    pub race: Box<Account<'info, Race>>,

    #[account(
        init,
        seeds = [TICKET_TAG, race.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<Ticket>() + 8
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
        token::authority = authority,
    )]
    pub user_token_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<PlaceBetCtx>, ix: PlaceBetIx) -> Result<()> {
    let ticket = &mut ctx.accounts.ticket;
    ticket.bump = ctx.bumps.ticket;
    ticket.authority = ctx.accounts.authority.key();
    ticket.race = ctx.accounts.race.key();
    ticket.hamster_id = ix.hamster_id;
    ticket.amount = ix.amount;

    let constraint = ctx
        .accounts
        .race
        .is_bet_available()
        .map_err(|_| ContractError::RaceNotAvailable)?;
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_vault.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        ix.amount,
    )?;

    ctx.accounts.race.handle_bet(ix.hamster_id, ix.amount)?;

    Ok(())
}
