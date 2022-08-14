use anchor_lang::prelude::*;

#[account]
pub struct OrbitDispute{
    pub dispute_transaction: Pubkey,
    pub favor: Pubkey,
    pub funder: Pubkey,
    pub state: OrbitDisputeState,
    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum OrbitDisputeState{
    Open,
    Resolved
}