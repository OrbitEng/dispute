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
        constraint = dispute_account.votes.len() < dispute_account.threshold
    )]
    pub dispute_account: Account<'info, OrbitDispute>,

    #[account(
        mut
    )]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = market_account.master_pubkey
    )]
    pub market_auth: Signer<'info>
}

pub fn vote_dispute_handler(ctx: Context<VoteDispute>, vote: DisputeSide) -> Result<()>{
    for vote in ctx.accounts.dispute_account.votes.iter(){
        if ctx.accounts.market_account.key() == vote.voter{
            return err!(DisputeErrors::AlreadyVoted)
        }
    };

    ctx.accounts.dispute_account.votes.push(
        DisputeVote{
            voter: ctx.accounts.market_account.key(),
            vote
        }
    );

    Ok(())
}

#[derive(Accounts)]
pub struct DisputeVerdict<'info>{
    #[account(
        mut,
        constraint = dispute_account.dispute_state == DisputeState::Open,
    )]
    pub dispute_account: Account<'info, OrbitDispute>,
}

pub fn dispute_verdict_handler(ctx: Context<DisputeVerdict>) -> Result<()>{
    let buyers = ctx.accounts.dispute_account.votes.iter().enumerate().filter(|v| v.1.vote == DisputeSide::Buyer).map(|v| (v.0, v.1.clone())).collect::<Vec<(usize, DisputeVote)>>();

    if buyers.len() > ctx.accounts.dispute_account.threshold/2{
        ctx.accounts.dispute_account.favor = ctx.accounts.dispute_account.buyer;
        for vote in buyers.iter(){
            if ctx.remaining_accounts[vote.0].key() != vote.1.voter{
                return err!(DisputeErrors::WrongRemainingAccounts);
            }
            let mut market_account = Account::<OrbitMarketAccount>::try_from(&ctx.remaining_accounts[vote.0].to_account_info()).expect("could not deserialize remaining account");
            market_account.dispute_discounts += 1;
            market_account.exit(ctx.program_id)?;
        }
        
    }else
    if ctx.accounts.dispute_account.votes.len() == ctx.accounts.dispute_account.threshold{
        ctx.accounts.dispute_account.favor = ctx.accounts.dispute_account.seller;
        
        for v in ctx.accounts.dispute_account.votes.iter().enumerate(){
            if v.1.vote != DisputeSide::Seller{
                
                if ctx.remaining_accounts[v.0].key() != v.1.voter{
                    return err!(DisputeErrors::WrongRemainingAccounts);
                }
                let mut market_account: Account<OrbitMarketAccount> = Account::<OrbitMarketAccount>::try_from(&ctx.remaining_accounts[v.0].to_account_info()).expect("could not deserialize remaining account");
                market_account.dispute_discounts += 1;
                market_account.exit(ctx.program_id)?;

            }
            
        }
    }else{
        return err!(DisputeErrors::CannotCloseDispute)
    }

    ctx.accounts.dispute_account.dispute_state = DisputeState::Resolved;
    Ok(())
}