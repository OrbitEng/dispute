use anchor_lang::{prelude::*, AccountsClose};
use market_accounts::structs::market_account::OrbitMarketAccount;
use crate::structs::dispute_struct::{OrbitDispute, DisputeState, DisputeSide, DisputeVote};

#[derive(Accounts)]
pub struct OpenDispute<'info>{
    #[account(
        init,
        payer = payer,
        space = 1000,

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
        constraint = 
            caller.key() == Pubkey::new(&[]) // make this physical
    )]
    pub caller: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>

}

pub fn open_dispute(ctx: Context<OpenDispute>, threshold: usize) -> Result<()>{
    ctx.accounts.new_dispute.favor = Pubkey::new_from_array([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.new_dispute.dispute_state = DisputeState::Open;
    ctx.accounts.new_dispute.dispute_transaction = ctx.accounts.in_transaction.key();
    ctx.accounts.new_dispute.funder = ctx.accounts.payer.key();

    ctx.accounts.new_dispute.threshold = threshold;
    Ok(())
}

#[derive(Accounts)]
pub struct CloseDispute<'info>{
    #[account(
        constraint = dispute_account.dispute_state == DisputeState::Resolved,
        has_one = funder
    )]
    pub dispute_account: Account<'info, OrbitDispute>,

    #[account(mut)]
    /// CHECK: we mutate a system account
    pub funder: SystemAccount<'info>,

    #[account(
        constraint = 
            caller.key() == Pubkey::new(&[]) // make this physical
    )]
    pub caller: Signer<'info>,
}

pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()>{
    ctx.accounts.dispute_account.close(ctx.accounts.funder.to_account_info())
}


/// CHECK: add constraints to market accounts (eg: must have n reputation)
#[derive(Accounts)]
pub struct VoteDispute<'info>{
    #[account(
        mut,
        constraint = dispute_account.dispute_state == DisputeState::Open
    )]
    pub dispute_account: Account<'info, OrbitDispute>,

    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = market_account.wallet
    )]
    pub market_wallet: Signer<'info>
}

pub fn vote_dispute(ctx: Context<VoteDispute>, vote: DisputeSide) -> Result<()>{
    ctx.accounts.dispute_account.voters.push(DisputeVote{
        voter: ctx.accounts.market_account.key(),
        vote
    });

    if ctx.accounts.dispute_account.voters.len() >= ctx.accounts.dispute_account.threshold{
        ctx.accounts.dispute_account.dispute_state = DisputeState::Resolved;
    }
    Ok(())
}