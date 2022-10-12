use anchor_lang::prelude::*;

declare_id!("GyXfrEB6iF7zDnrvte9WVjMW8fgTG8URhWvWYADS39dZ");

pub mod structs;
pub mod accessors;
pub mod errors;

pub use errors::*;
pub use structs::*;
pub use accessors::*;

#[program]
pub mod dispute {
    use super::*;
    
    pub fn open_dispute(ctx: Context<OpenDispute>, threshold: u8) -> Result<()> {
        open_dispute_handler(ctx, threshold)
    }
    pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()> {
        close_dispute_handler(ctx)
    }
    pub fn vote_dispute(ctx: Context<VoteDispute>, vote: DisputeSide) -> Result<()> {
        vote_dispute_handler(ctx, vote)
    }
    pub fn cast_verdict<'a>(ctx: Context<'_, '_, '_, 'a, DisputeVerdict<'a>>) -> Result<()>{
        dispute_verdict_handler(ctx)
    }
}