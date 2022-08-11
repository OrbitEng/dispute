use anchor_lang::{prelude::*, AccountsClose};
use crate::structs::dispute_struct::{OrbitDispute, OrbitDisputeState};

#[derive(Accounts)]
pub struct OpenPhysicalDispute<'info>{
    #[account(
        mut,
        seeds = [
            b"physical_dispute",
            phys_transaction.key().as_ref()
        ],
        bump
    )]
    pub new_dispute: Account<'info, OrbitDispute>,

    /// CHECK: we dont check this because we would need to import the struct from physical transactions subcrate
    /// and that would lead to circular dependencies :P
    /// will fix soon just not rn.
    pub phys_transaction: AccountInfo<'info>,

    pub opener: Signer<'info>
}


#[derive(Accounts)]
pub struct ClosePhysicalDispute<'info>{
    #[account(
        constraint = phys_dispute.state == OrbitDisputeState::Closed,
        has_one = funder
    )]
    pub phys_dispute: Account<'info, OrbitDispute>,

    #[account(mut)]
    /// CHECK: we dont check this because theres nothing to check it against
    /// its a system account
    pub funder: AccountInfo<'info>,
}

pub fn open_physical_dispute(ctx: Context<OpenPhysicalDispute>) -> Result<()>{
    ctx.accounts.new_dispute.favor = Pubkey::new_from_array([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.new_dispute.state = OrbitDisputeState::Open;
    ctx.accounts.new_dispute.dispute_transaction = ctx.accounts.phys_transaction.key();
    Ok(())
}

pub fn close_physical_dispute(ctx: Context<ClosePhysicalDispute>) -> Result<()>{
    ctx.accounts.phys_dispute.close(ctx.accounts.funder.to_account_info())
}

#[derive(Accounts)]
pub struct ResolvePhysicalDispute<'info>{
    #[account(mut)]
    pub phys_dispute: Account<'info, OrbitDispute>
}

// we do this as middle step
pub fn mark_dispute_resolved(ctx: Context<ResolvePhysicalDispute>) -> Result<()>{
    ctx.accounts.phys_dispute.state = OrbitDisputeState::Closed;
    Ok(())
}