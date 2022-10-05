use anchor_lang::prelude::*;

#[account]
pub struct OrbitDispute{
    pub dispute_transaction: Pubkey,
    pub favor: u64,
    pub funder: Pubkey,
    pub dispute_state: DisputeState,

    pub buyer: u64,
    pub seller: u64,
    
    pub buyer_votes: Vec<Pubkey>,
    pub seller_votes: Vec<Pubkey>,

    pub threshold: usize,
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