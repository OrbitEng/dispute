use anchor_lang::{prelude::*, AccountsClose};
use market_accounts::{structs::market_account::OrbitMarketAccount, program::OrbitMarketAccounts};
use orbit_addresses::PHYSICAL_ADDRESS;
use crate::{
    structs::dispute_struct::{
        OrbitDispute,
        DisputeState,
        DisputeSide
    },
    DisputeErrors, program::Dispute
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
        owner = caller_program.key()
    )]
    pub in_transaction: AccountInfo<'info>,

    pub buyer: Box<Account<'info, OrbitMarketAccount>>,

    pub seller: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        seeds = [
            b"market_authority"
        ],
        seeds::program = caller_program.key(),
        bump
    )]
    pub caller_auth: Signer<'info>,

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

pub fn open_dispute_handler(ctx: Context<OpenDispute>, threshold: u8) -> Result<()>{
    if (threshold % 2) == 0{
        return err!(DisputeErrors::EvenThreshold)
    }

    ctx.accounts.new_dispute.favor = 0;
    ctx.accounts.new_dispute.dispute_state = DisputeState::Open;
    ctx.accounts.new_dispute.dispute_transaction = ctx.accounts.in_transaction.key();
    ctx.accounts.new_dispute.funder = ctx.accounts.payer.key();
    ctx.accounts.new_dispute.buyer = ctx.accounts.buyer.voter_id;
    ctx.accounts.new_dispute.seller = ctx.accounts.seller.voter_id;

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
            b"market_authority"
        ],
        seeds::program = caller.key(),
        bump
    )]
    pub caller_auth: Signer<'info>,

    #[account(
        constraint =
            caller.key() == Pubkey::new(PHYSICAL_ADDRESS)
    )]
    /// CHECK: there are constraints the linter can kill itself
    pub caller: AccountInfo<'info>
}

pub fn close_dispute_handler(ctx: Context<CloseDispute>) -> Result<()>{
    ctx.accounts.dispute_account.dispute_state = DisputeState::Closed;
    ctx.accounts.dispute_account.close(ctx.accounts.funder.to_account_info())
}


/// CHECK: add constraints to market accounts (eg: must have n reputation)
#[derive(Accounts)]
pub struct VoteDispute<'info>{
    #[account(
        mut,
        constraint = dispute_account.dispute_state == DisputeState::Open,
        constraint = (dispute_account.buyer_votes.len() + dispute_account.seller_votes.len()) < dispute_account.threshold
    )]
    pub dispute_account: Account<'info, OrbitDispute>,

    #[account(
        mut,
        constraint = (market_account.voter_id != dispute_account.buyer) && (market_account.voter_id != dispute_account.seller),
        constraint = market_account.transactions > 3,
        constraint = (Clock::get()?.unix_timestamp - market_account.account_created) > 604_800, // greater than a week

        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump,
        seeds::program = market_accounts::ID
    )]
    pub market_account: Box<Account<'info, OrbitMarketAccount>>,

    pub wallet: Signer<'info>
}

pub fn vote_dispute_handler(ctx: Context<VoteDispute>, vote: DisputeSide) -> Result<()>{
    if ctx.accounts.dispute_account.buyer_votes.iter().find(|&&v| v == ctx.accounts.market_account.key()).is_some(){
        return err!(DisputeErrors::AlreadyVoted)
    };
    if ctx.accounts.dispute_account.seller_votes.iter().find(|&&v| v == ctx.accounts.market_account.key()).is_some(){
        return err!(DisputeErrors::AlreadyVoted)
    };
    match vote{
        DisputeSide::Buyer => ctx.accounts.dispute_account.buyer_votes.push(ctx.accounts.market_account.key()),
        DisputeSide::Seller => ctx.accounts.dispute_account.seller_votes.push(ctx.accounts.market_account.key())
    }

    Ok(())
}

#[derive(Accounts)]
pub struct DisputeVerdict<'info>{
    #[account(
        mut,
        constraint = dispute_account.dispute_state == DisputeState::Open,
    )]
    pub dispute_account: Account<'info, OrbitDispute>,

    #[account(
        seeds = [
            b"market_authority"
        ],
        bump
    )]
    pub dispute_auth: SystemAccount<'info>,

    pub dispute_program: Program<'info, Dispute>,
    
    pub market_accounts_program: Program<'info, OrbitMarketAccounts>
}

/// remaining accounts is buyers[] then sellers[]
pub fn dispute_verdict_handler<'a>(ctx: Context<'_, '_, '_, 'a, DisputeVerdict<'a>>) -> Result<()>{
    let thresh = ctx.accounts.dispute_account.threshold/2;

    if ctx.accounts.dispute_account.buyer_votes.len() > thresh{
        ctx.accounts.dispute_account.favor = ctx.accounts.dispute_account.buyer;
        for vote in ctx.accounts.dispute_account.buyer_votes.iter().enumerate(){
            if ctx.remaining_accounts[vote.0].key() != *vote.1{
                return err!(DisputeErrors::WrongRemainingAccounts);
            }
        }
        let mut cpi_ctx = 
            CpiContext::new(
                ctx.accounts.market_accounts_program.to_account_info(),
                market_accounts::cpi::accounts::MarketAccountMultipleUpdateInternal{
                    caller_auth: ctx.accounts.dispute_auth.to_account_info(),
                    caller: ctx.accounts.dispute_program.to_account_info()
                }
            );
        cpi_ctx.remaining_accounts = ctx.remaining_accounts[0..ctx.accounts.dispute_account.buyer_votes.len()].to_vec();
        market_accounts::cpi::increment_dispute_discounts_multiple(
            cpi_ctx
        )?;
        
    }else
    if ctx.accounts.dispute_account.seller_votes.len() > thresh{
        let buyer_len = ctx.accounts.dispute_account.buyer_votes.len();
        ctx.accounts.dispute_account.favor = ctx.accounts.dispute_account.seller;
        
        for vote in ctx.accounts.dispute_account.seller_votes.iter().enumerate(){
            if ctx.remaining_accounts[vote.0 + buyer_len].key() != *vote.1{
                return err!(DisputeErrors::WrongRemainingAccounts);
            }
        }
        let mut cpi_ctx = CpiContext::new(
            ctx.accounts.market_accounts_program.to_account_info(),
            market_accounts::cpi::accounts::MarketAccountMultipleUpdateInternal{
                caller_auth: ctx.accounts.dispute_auth.to_account_info(),
                caller: ctx.accounts.dispute_program.to_account_info()
            }
        );
        cpi_ctx.remaining_accounts = ctx.remaining_accounts[buyer_len..].to_vec();
        market_accounts::cpi::increment_dispute_discounts_multiple(
            cpi_ctx
        )?;
    }else{
        return err!(DisputeErrors::CannotCloseDispute)
    }

    ctx.accounts.dispute_account.dispute_state = DisputeState::Resolved;
    Ok(())
}