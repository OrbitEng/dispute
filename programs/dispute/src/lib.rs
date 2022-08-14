use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod structs;
pub mod accessors;

pub use structs::*;
pub use accessors::*;

#[program]
pub mod dispute {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn open_dispute(ctx: Context<OpenDispute>) -> Result<()> {
        accessors::open_dispute(ctx)
    }
    pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()> {
        accessors::close_dispute(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        seeds = [b"dispute_auth"],
        bump,
        space = 64,
        payer = payer
    )]
    /// CHECK: It's a system account
    pub dispute_market_auth: AccountInfo<'info>,

    #[account(mut)]
    pub payer : Signer<'info>,

    pub system_program: Program<'info, System>
}
