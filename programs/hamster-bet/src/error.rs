use anchor_lang::prelude::*;

#[error_code]
pub enum ContractError {
    #[msg("Calculation Error.")]
    CalcError,

    #[msg("MathOverflow.")]
    MathOverflow,

    #[msg("Invalid address.")]
    InvalidAddress,

    #[msg("Invalid authority.")]
    InvalidAuthority,

    #[msg("Invalid token.")]
    InvalidToken,

    #[msg("Race not available.")]
    RaceNotAvailable,

    #[msg("Hamster Not Exist.")]
    HamsterNotExist,

    #[msg("HamsterWinner Not Exist.")]
    HamsterWinnerNotExist,

    #[msg("Invalid Cashout.")]
    InvalidCashout,

    #[msg("Invalid Withdraw.")]
    InvalidWithdraw,

    #[msg("Invalid Ticket.")]
    InvalidTicket,
}
