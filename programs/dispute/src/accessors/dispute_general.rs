use anchor_lang::{prelude::*, AccountsClose};
use market_accounts::structs::market_account::OrbitMarketAccount;
use orbit_addresses::PHYSICAL_ADDRESS;
use crate::{
    structs::dispute_struct::{
        OrbitDispute,
        DisputeState,
        DisputeSide,
        DisputeVote
    },
    DisputeErrors
};

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
    #[account(
        constraint = *in_transaction.owner == caller_program.key()
    )]
    pub in_transaction: AccountInfo<'info>,

    pub buyer: Account<'info, OrbitMarketAccount>,

    pub seller: Account<'info, OrbitMarketAccount>,

    #[account(
        seeds = [
            b"dispute_auth"
        ],
        seeds::program = caller_program.key(),
        bump
    )]
    pub dispute_auth: Signer<'info>,

    #[account(
        constraint = 
            (caller_program.key() == Pubkey::new(orbit_addresses::PHYSICAL_ADDRESS))
    )]
    /// CHECK: program calling. you will see why
    pub caller_program: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>

}

pub fn open_dispute(ctx: Context<OpenDispute>, threshold: u8) -> Result<()>{
    if (threshold % 2) == 0{
        return err!(DisputeErrors::EvenThreshold)
    }

    ctx.accounts.new_dispute.favor = Pubkey::new_from_array([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    ctx.accounts.new_dispute.dispute_state = DisputeState::Open;
    ctx.accounts.new_dispute.dispute_transaction = ctx.accounts.in_transaction.key();
    ctx.accounts.new_dispute.funder = ctx.accounts.payer.key();
    ctx.accounts.new_dispute.buyer = ctx.accounts.buyer.key();
    ctx.accounts.new_dispute.seller = ctx.accounts.seller.key();

    ctx.accounts.new_dispute.threshold = threshold as usize;
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
        seeds = [
            b"dispute_auth"
        ],
        seeds::program = caller.key(),
        bump
    )]
    pub dispute_auth: Signer<'info>,

    #[account(
        constraint =
            caller.key() == Pubkey::new(PHYSICAL_ADDRESS)
    )]
    /// CHECK: there are constraints the linter can kill itself
    pub caller: AccountInfo<'info>
}

pub fn close_dispute(ctx: Context<CloseDispute>) -> Result<()>{
    ctx.accounts.dispute_account.dispute_state = DisputeState::Closed;
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
        address = market_account.master_pubkey
    )]
    pub market_auth: Signer<'info>
}

pub fn vote_dispute(ctx: Context<VoteDispute>, vote: DisputeSide) -> Result<()>{
    ctx.accounts.dispute_account.voters.push(DisputeVote{
        voter: ctx.accounts.market_account.key(),
        vote
    });

    if ctx.accounts.dispute_account.voters.len() >= ctx.accounts.dispute_account.threshold{
        let mut side = 0;
        for vote in ctx.accounts.dispute_account.voters.iter(){
            match vote.vote{
                DisputeSide::Buyer => side += 1,
                DisputeSide::Seller => side -= 1
            }
        }
        if side > 0 {
            ctx.accounts.dispute_account.favor = ctx.accounts.dispute_account.buyer;
        }else
        if side < 0{
            ctx.accounts.dispute_account.favor = ctx.accounts.dispute_account.seller;
        }else{
            return err!(DisputeErrors::CannotCloseDispute)
        }
        ctx.accounts.dispute_account.dispute_state = DisputeState::Resolved;
    }
    Ok(())
}