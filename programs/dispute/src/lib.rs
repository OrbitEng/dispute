use anchor_lang::prelude::*;

declare_id!("HPDHvMeJ2Vepn5jEmsWNvEJwgFhY3WoNFZapKxQxQhDK");

pub mod structs;
pub mod accessors;

pub use structs::*;
pub use accessors::*;

#[program]
pub mod dispute {
    use super::*;
    
    pub fn open_dispute(ctx: Context<OpenDispute>, threshold: usize) -> Result<()> {
        accessors::open_dispute(ctx, threshold)
    }
    pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()> {
        accessors::close_dispute(ctx)
    }
    pub fn vote_dispute(ctx: Context<VoteDispute>, vote: DisputeSide) -> Result<()> {
        accessors::vote_dispute(ctx, vote)
    }
}