use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod structs;
pub mod accessors;

#[program]
pub mod dispute {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
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
    pub dispute_market_auth: AccountInfo<'info>,

    #[account(mut)]
    pub payer : Signer<'info>,

    pub system_program: Program<'info, System>
}
