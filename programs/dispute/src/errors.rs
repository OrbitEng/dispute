use anchor_lang::prelude::*;

#[error_code]
pub enum DisputeErrors{
    #[msg("threshold must be an odd number of people")]
    EvenThreshold,
    #[msg("can't close dispute")]
    CannotCloseDispute,
}