use anchor_lang::prelude::*;

#[error_code]
pub enum RouterError {
    #[msg("ATA not found")]
    AtaNotFound,
    #[msg("Receiver not found")]
    ReceiverNotFound,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid data")]
    InvalidData,
    #[msg("Accounts not enough")]
    InsufficientAccounts,
}
