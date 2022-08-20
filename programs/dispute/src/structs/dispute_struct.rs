use anchor_lang::prelude::*;

#[account]
pub struct OrbitDispute{
    pub dispute_transaction: Pubkey,
    pub favor: Pubkey,
    pub funder: Pubkey,
    pub dispute_state: DisputeState,

    pub buyer: Pubkey,
    pub seller: Pubkey,
    
    pub voters: Vec<DisputeVote>,

    pub threshold: usize,
}

/// CHECK: yeah each struct is 40. there's stuff I should do about it. I'm not going to.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub struct DisputeVote{
    pub voter: Pubkey,
    pub vote: DisputeSide
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum DisputeSide{
    Seller,
    Buyer
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum DisputeState{
    Open,
    Closed,
    Resolved
}