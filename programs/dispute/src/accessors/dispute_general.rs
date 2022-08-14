use anchor_lang::{prelude::*, AccountsClose};
use crate::structs::dispute_struct::{OrbitDispute, OrbitDisputeState};

#[derive(Accounts)]
pub struct OpenDispute<'info>{
    #[account(
        init,
        payer = payer,
        space = 200,

        seeds = [
            b"dispute_account",
            in_transaction.key().as_ref()
        ],
        bump,
    )]
    pub new_dispute: Account<'info, OrbitDispute>,

    /// CHECK: accountinfo due to circular dependencies
    /// we do checks on initial contract
    /// and this contract is cpi auth bound
    pub in_transaction: AccountInfo<'info>,

    #[account(
        seeds = [
            b"physical_auth"
        ],
        seeds::program = Pubkey::new(orbit_addresses::PHYSICAL_ADDRESS),
        bump
    )]
    pub caller: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>

}



pub fn open_dispute(ctx: Context<OpenDispute>) -> Result<()>{
    ctx.accounts.new_dispute.favor = Pubkey::new_from_array([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.new_dispute.state = OrbitDisputeState::Open;
    ctx.accounts.new_dispute.dispute_transaction = ctx.accounts.in_transaction.key();
    ctx.accounts.new_dispute.funder = ctx.accounts.payer.key();
    Ok(())
}

#[derive(Accounts)]
pub struct CloseDispute<'info>{
    #[account(
        constraint = phys_dispute.state == OrbitDisputeState::Closed,
        has_one = funder
    )]
    pub phys_dispute: Account<'info, OrbitDispute>,

    #[account(mut)]
    /// CHECK: we dont check this because theres nothing to check it against
    /// its a system account
    pub funder: SystemAccount<'info>,

    #[account(
        seeds = [
            b"physical_auth"
        ],
        seeds::program = Pubkey::new(orbit_addresses::PHYSICAL_ADDRESS),
        bump
    )]
    pub caller: Signer<'info>,
}

pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()>{
    ctx.accounts.phys_dispute.close(ctx.accounts.funder.to_account_info())
}