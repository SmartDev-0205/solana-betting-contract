use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Ticket {
    pub bump: u8,
    pub authority: Pubkey, // user's wallet
    pub race: Pubkey,
    pub hamster_id: u8,
    pub amount: u64,
    pub settled: bool, // if winner and cashout, set true
    pub reserved: [u128; 1],
    pub claim: u64,
}
